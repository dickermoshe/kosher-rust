// Initialize the test environment
import 'dart:math';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:jni/jni.dart';
import 'package:test_with_java/src/rust/frb_generated.dart';

late Random random;
int defaultSeed() => DateTime.now().millisecondsSinceEpoch;

Future<void> init({int? seed}) async {
  print("Seed: $seed");
  random = Random(seed);
  // Initialize Rust library
  await RustLib.init(
      externalLibrary: await loadExternalLibrary(ExternalLibraryLoaderConfig(
    stem: 'test_with_java',
    ioDirectory: '../target/release/',
    webPrefix: 'pkg/',
  )));
  // Initialize Java runtime
  Jni.spawn(classPath: ["./java/target/zmanim-2.6.0-SNAPSHOT.jar"]);
}
