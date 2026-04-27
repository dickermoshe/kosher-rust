This Dart project sereves as the test harness for the the Rust Zmanim project.
The Dart programming language has excellent support for interop with Java and Rust. 
Therefor it is the perfect language to test the Rust port of the KosherJava Zmanim library.

## Project Structure

```
test/
├── bin/ # The test harness
├── java/ # KosherJava sources
├── rust/ # A small Rust project that wraps the Rust Zmanim library
├── lib/ # Generated Dart bindings for KosherJava, the Rust Zmanim library and some common utilities
```

## Update KosherJava

To pull in the latest version of the KosherJava Zmanim library, run the following command:

Replace `<branch-name>` with the branch of the KosherJava Zmanim library to pull from.

```
git subtree pull --prefix=test/java  https://github.com/KosherJava/zmanim <branch-name> --squash
```

## Pre-requisites

To run the tests, you need to have the following prerequisites installed:

```
# Cargo Prerequisites
cargo install cargo-expand
cargo install flutter_rust_bridge_codegen

# Dart Prerequisites
dart pub get
dart run jni:setup

# Java Prerequisites
# Ensure Java & Maven is installed and JAVA_HOME is set
# Ensure that `JAVA_HOME\bin\server` is in the PATH on Windows
```

## Generate the Rust Bindings code

To generate the Dart bindings for the Rust Zmanim library, run the following command:

```
flutter_rust_bridge_codegen generate
```

## Generate the Java Bindings code

To generate the Dart bindings for the KosherJava Zmanim library, run the following command:

```
dart run jnigen.dart
```

## Build the KosherJava & Rust libraries

To build the KosherJava `.jar` & Rust `.so` libraries, run the following command:

```
dart run build.dart
```

## Run the tests

To run the tests, run the following command:

```
dart test
```
