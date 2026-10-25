import { useMemo, useState } from "react";
import {
  allZmanim,
  amudYomiDirshu,
  dafYomiBavli,
  dafYomiYerushalmi,
  HebrewDate,
  mishnaYomis,
  pirkeiAvos,
  tehillimMonthly,
  Zmanim,
  type Category,
  type GregorianDate,
  type LocationOptions,
  type Region,
  type Zman,
  type ZmanTime,
} from "kosher-rust";

interface City {
  id: string;
  name: string;
  location: LocationOptions;
  timeZone: string;
  region: Region;
}

const CITIES: City[] = [
  { id: "lakewood", name: "Lakewood", location: { latitude: 40.0961, longitude: -74.2224, elevation: 30 }, timeZone: "America/New_York", region: "diaspora" },
  { id: "jerusalem", name: "Jerusalem", location: { latitude: 31.778, longitude: 35.235, elevation: 754 }, timeZone: "Asia/Jerusalem", region: "israel" },
  { id: "bnei-brak", name: "Bnei Brak", location: { latitude: 32.0807, longitude: 34.8338, elevation: 30 }, timeZone: "Asia/Jerusalem", region: "israel" },
  { id: "london", name: "London", location: { latitude: 51.5074, longitude: -0.1278, elevation: 20 }, timeZone: "Europe/London", region: "diaspora" },
  { id: "los-angeles", name: "Los Angeles", location: { latitude: 34.0522, longitude: -118.2437, elevation: 90 }, timeZone: "America/Los_Angeles", region: "diaspora" },
  { id: "johannesburg", name: "Johannesburg", location: { latitude: -26.2041, longitude: 28.0473, elevation: 1753 }, timeZone: "Africa/Johannesburg", region: "diaspora" },
  { id: "melbourne", name: "Melbourne", location: { latitude: -37.8136, longitude: 144.9631, elevation: 30 }, timeZone: "Australia/Melbourne", region: "diaspora" },
  { id: "tromso", name: "Tromsø", location: { latitude: 69.6492, longitude: 18.9553, elevation: 0 }, timeZone: "Europe/Oslo", region: "diaspora" },
];

/** The times most people look for, in the order of the day. */
const MAIN: Zman[] = [
  "alos72Minutes",
  "misheyakir10Point2Degrees",
  "sunrise",
  "sofZmanShmaMga72Minutes",
  "sofZmanShmaGra",
  "sofZmanTfilaGra",
  "chatzosHayom",
  "minchaGedolaGra",
  "minchaKetanaGra",
  "plagHaminchaGra",
  "candleLighting",
  "sunset",
  "tzaisGeonim8Point5Degrees",
  "tzais72Minutes",
  "chatzosHalayla",
];

const CATEGORY_NAMES: Record<Category, string> = {
  twilight: "Twilight",
  alos: "Dawn",
  misheyakir: "Tallis and tefillin",
  netz: "Sunrise",
  sofZmanShma: "Latest Shema",
  sofZmanTefila: "Latest Shacharis",
  sofZmanAchilasChametz: "Latest to eat chametz",
  sofZmanBiurChametz: "Latest to burn chametz",
  chatzosHayom: "Midday",
  minchaGedola: "Earliest Mincha",
  plagHamincha: "Plag hamincha",
  samuchLeMinchaKetana: "Samuch le-mincha ketana",
  minchaKetana: "Mincha ketana",
  beinHashmashos: "Bein hashmashos",
  candleLighting: "Candle lighting",
  shkiya: "Sunset",
  tzais: "Nightfall",
  kidushLevana: "Kiddush Levana",
  chatzosHalayla: "Midnight",
};

const UNAVAILABLE = {
  allDay: "The sun stays too high today",
  allNight: "The sun stays too low today",
  notApplicable: "Not today",
  timeZoneRequired: "Needs a time zone",
  dateOutOfRange: "Out of range",
  invalidDegrees: "Invalid angle",
  internal: "Could not be calculated",
} as const;

function fromUrl(): { cityId: string; date: string } {
  const params = new URLSearchParams(window.location.search);
  const cityId = CITIES.some((c) => c.id === params.get("city")) ? params.get("city")! : "jerusalem";
  const city = CITIES.find((c) => c.id === cityId)!;
  return { cityId, date: params.get("date") ?? HebrewDate.today(city.timeZone).toJSON() };
}

/** The `YYYY-MM-DD` day `text` as an object. */
function fromIso(text: string): GregorianDate {
  const [year, month, day] = text.split("-").map(Number);
  return { year: year!, month: month!, day: day! };
}

function shift(date: string, days: number): string {
  return HebrewDate.from(fromIso(date)).addDays(days).toJSON();
}

function clock(time: Date, timeZone: string): string {
  return time.toLocaleTimeString("en-US", { timeZone, hour: "numeric", minute: "2-digit", second: "2-digit" });
}

export function App() {
  const initial = useMemo(fromUrl, []);
  const [cityId, setCityId] = useState(initial.cityId);
  const [date, setDate] = useState(initial.date);
  const [allOpinions, setAllOpinions] = useState(false);
  const city = CITIES.find((c) => c.id === cityId)!;

  const choose = (next: { cityId?: string; date?: string }) => {
    const params = new URLSearchParams({ city: next.cityId ?? cityId, date: next.date ?? date });
    window.history.replaceState(null, "", `?${params}`);
    if (next.cityId) setCityId(next.cityId);
    if (next.date) setDate(next.date);
  };

  const day = useMemo(() => {
    try {
      return HebrewDate.from(fromIso(date));
    } catch {
      return null;
    }
  }, [date]);

  const zmanim = useMemo(
    () => (day ? new Zmanim({ location: city.location, timeZone: city.timeZone, date: day }) : null),
    [city, day],
  );

  if (!day || !zmanim) {
    return (
      <main className="page">
        <p className="error">“{date}” is not a date the calendar covers.</p>
      </main>
    );
  }

  const times = zmanim.all();
  const candles = day.hasCandleLighting(city.region);
  const shown = allOpinions
    ? times
    : MAIN.map((id) => times.find((t) => t.zman.id === id)!).filter((t) => t.zman.id !== "candleLighting" || candles);
  const parsha = day.parsha(city.region);
  const specialParsha = day.specialParsha(city.region);
  const gregorian = new Date(Date.UTC(day.gregorian.year, day.gregorian.month - 1, day.gregorian.day));
  const molad = day.molad();

  return (
    <main className="page">
      <header className="hero">
        <p className="eyebrow">
          {gregorian.toLocaleDateString("en-US", { timeZone: "UTC", weekday: "long", year: "numeric", month: "long", day: "numeric" })}
        </p>
        <h1 data-testid="hebrew-date">{day.toString()}</h1>
        <p className="hebrew" lang="he" dir="rtl">
          {day.month.hebrew}
        </p>
        <div className="chips" data-testid="special-days">
          {day.specialDays(city.region).map((special) => (
            <span key={special.id} className={special.isAssurBemelacha ? "chip yomtov" : special.isFastDay ? "chip fast" : "chip"}>
              {special.name}
              {special.day ? ` ${special.day}` : ""}
            </span>
          ))}
          {parsha && <span className="chip parsha">Parshas {parsha.name}</span>}
          {specialParsha && <span className="chip parsha">Shabbos {specialParsha.name}</span>}
        </div>
      </header>

      <nav className="controls">
        <label>
          <span>Place</span>
          <select data-testid="city" value={cityId} onChange={(e) => choose({ cityId: e.target.value })}>
            {CITIES.map((c) => (
              <option key={c.id} value={c.id}>
                {c.name}
              </option>
            ))}
          </select>
        </label>
        <div className="date-picker">
          <button aria-label="Previous day" onClick={() => choose({ date: shift(date, -1) })}>
            ‹
          </button>
          <input data-testid="date" type="date" value={date} onChange={(e) => e.target.value && choose({ date: e.target.value })} />
          <button aria-label="Next day" onClick={() => choose({ date: shift(date, 1) })}>
            ›
          </button>
          <button className="today" onClick={() => choose({ date: HebrewDate.today(city.timeZone).toJSON() })}>
            Today
          </button>
        </div>
        <label className="toggle">
          <input type="checkbox" checked={allOpinions} onChange={(e) => setAllOpinions(e.target.checked)} />
          <span>Every opinion</span>
        </label>
      </nav>

      <div className="grid">
        <section className="card zmanim">
          <h2>
            Zmanim <small>{city.timeZone.replace(/_/g, " ")}</small>
          </h2>
          {allOpinions ? <Grouped times={shown} timeZone={city.timeZone} /> : <Rows times={shown} timeZone={city.timeZone} />}
        </section>

        <aside className="side">
          <section className="card">
            <h2>Learning</h2>
            <dl className="learning" data-testid="learning">
              <Item term="Daf Yomi" value={dafYomiBavli(day)} />
              <Item term="Amud Yomi" value={amudYomiDirshu(day)} />
              <Item term="Yerushalmi" value={dafYomiYerushalmi(day)} />
              <Item term="Mishna Yomis" value={mishnaYomis(day)} />
              <Item term="Tehillim" value={tehillimMonthly(day)} />
              <Item term="Pirkei Avos" value={pirkeiAvos(day, city.region)} />
            </dl>
          </section>
          <section className="card">
            <h2>The month</h2>
            <dl className="learning">
              <Item
                term="Molad"
                value={`${["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Shabbos"][molad.weekday]} ${molad.hours}:${String(molad.minutes).padStart(2, "0")} and ${molad.chalakim} chalakim`}
              />
              <Item term="Year" value={`${day.year}, ${day.hebrewYear.isLeap ? "a leap year" : "a common year"} of ${day.hebrewYear.days} days`} />
              <Item term="Day of year" value={day.dayOfYear} />
            </dl>
          </section>
          <p className="footnote">
            Times for {city.location.latitude.toFixed(4)}, {city.location.longitude.toFixed(4)}. Calculated in your browser with{" "}
            <a href="https://github.com/dickermoshe/kosher-rust">kosher-rust</a>.
          </p>
        </aside>
      </div>
    </main>
  );
}

function Item({ term, value }: { term: string; value: { toString(): string } | undefined }) {
  return (
    <div>
      <dt>{term}</dt>
      <dd>{value === undefined ? "—" : String(value)}</dd>
    </div>
  );
}

function Rows({ times, timeZone }: { times: ZmanTime[]; timeZone: string }) {
  return (
    <ul className="rows" data-testid="zmanim">
      {times.map(({ zman, time, unavailable }) => (
        <li key={zman.id} data-zman={zman.id}>
          <span className="name">{zman.name}</span>
          <span className={time ? "time" : "time none"}>{time ? clock(time, timeZone) : UNAVAILABLE[unavailable!]}</span>
        </li>
      ))}
    </ul>
  );
}

function Grouped({ times, timeZone }: { times: ZmanTime[]; timeZone: string }) {
  const categories = [...new Set(allZmanim().map((z) => z.category))];
  return (
    <>
      {categories.map((category) => (
        <div key={category} className="group">
          <h3>{CATEGORY_NAMES[category]}</h3>
          <Rows times={times.filter((t) => t.zman.category === category && !t.zman.deprecated)} timeZone={timeZone} />
        </div>
      ))}
    </>
  );
}
