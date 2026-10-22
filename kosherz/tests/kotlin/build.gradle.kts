// Compiles the generated Kotlin sources for a desktop JVM together with the smoke test.
// `-PlibraryPath=` is the java.library.path holding libkosherz and libkosherz_jni.
plugins {
    kotlin("jvm") version "2.2.20"
    application
}

repositories {
    mavenCentral()
}

kotlin {
    sourceSets {
        named("main") {
            kotlin.srcDir("../../generated/kotlin/src/main/kotlin")
        }
    }
}

application {
    mainClass.set("SmokeKt")
}

tasks.named<JavaExec>("run") {
    systemProperty("java.library.path", providers.gradleProperty("libraryPath").getOrElse(""))
}
