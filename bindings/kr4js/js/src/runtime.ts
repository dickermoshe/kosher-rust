import * as glue from "./wasm/kr4js.js";
import { offsetAt } from "./timezone.js";

/** Instantiates the WebAssembly from `module`, for the entries that load it as the package loads. */
export function start(module: () => WebAssembly.Module | BufferSource): void {
  try {
    glue.initSync({ module: module() });
  } catch (error) {
    throw new Error(
      "kr4js could not compile its WebAssembly here. Where compiling it at runtime isn't " +
        "allowed, as in Cloudflare Workers, import @dickermoshe/kr4js/slim and pass `init` " +
        "the module imported from @dickermoshe/kr4js/kr4js.wasm.",
      { cause: error },
    );
  }
  glue.setTimeZoneResolver(offsetAt);
}

let loading: Promise<void> | undefined;

/**
 * Instantiates the WebAssembly from `source`, for the slim entry. Later calls, including ones made
 * while the first is still loading, give the first call's promise. After a failure the next call
 * tries again.
 */
export function load(source: glue.InitInput | Promise<glue.InitInput>): Promise<void> {
  loading ??= glue.default({ module_or_path: source }).then(
    () => glue.setTimeZoneResolver(offsetAt),
    (error: unknown) => {
      loading = undefined;
      throw error;
    },
  );
  return loading;
}
