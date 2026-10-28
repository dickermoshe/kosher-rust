// Finds the library the build hook bundled, for flutter_rust_bridge to open.
//
// flutter_rust_bridge's own loader does not find a build hook's library outside Flutter
// (https://github.com/fzyzcjy/flutter_rust_bridge/issues/3379). Dart opens a code asset when one
// of its `@Native` functions is first used, but it does not say where the file is. So this looks
// up one symbol through the asset, and asks the operating system which loaded file holds that
// address.

import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// The asset the hook bundles, named by `FlutterRustBridgeNativeAssetsBuilder`.
@Native<Int32 Function()>(
  symbol: 'frb_get_rust_content_hash',
  assetId: 'package:kosher_rust/src/rust/frb_generated.io.dart',
)
external int _contentHash();

/// The kosher-rust library bundled with this package.
ExternalLibrary bundledLibrary() {
  final address = Native.addressOf<NativeFunction<Int32 Function()>>(
    _contentHash,
  );
  return ExternalLibrary.open(_pathOf(address.cast()));
}

String _pathOf(Pointer<Void> address) =>
    Platform.isWindows ? _windowsPathOf(address) : _posixPathOf(address);

final class _DlInfo extends Struct {
  external Pointer<Utf8> fileName;
  external Pointer<Void> base;
  external Pointer<Utf8> symbolName;
  external Pointer<Void> symbolAddress;
}

String _posixPathOf(Pointer<Void> address) {
  final dladdr = DynamicLibrary.process()
      .lookupFunction<
        Int32 Function(Pointer<Void>, Pointer<_DlInfo>),
        int Function(Pointer<Void>, Pointer<_DlInfo>)
      >('dladdr');
  final info = calloc<_DlInfo>();
  try {
    if (dladdr(address, info) == 0) {
      throw StateError(
        'The kosher_rust library is loaded, but its file was not found.',
      );
    }
    return info.ref.fileName.toDartString();
  } finally {
    calloc.free(info);
  }
}

String _windowsPathOf(Pointer<Void> address) {
  final kernel32 = DynamicLibrary.open('kernel32.dll');
  final getModuleHandleEx = kernel32
      .lookupFunction<
        Int32 Function(Uint32, Pointer<Void>, Pointer<Pointer<Void>>),
        int Function(int, Pointer<Void>, Pointer<Pointer<Void>>)
      >('GetModuleHandleExW');
  final getModuleFileName = kernel32
      .lookupFunction<
        Uint32 Function(Pointer<Void>, Pointer<Utf16>, Uint32),
        int Function(Pointer<Void>, Pointer<Utf16>, int)
      >('GetModuleFileNameW');

  // GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT
  const fromAddress = 0x4 | 0x2;
  const capacity = 32768;
  final module = calloc<Pointer<Void>>();
  final name = calloc<Uint16>(capacity).cast<Utf16>();
  try {
    if (getModuleHandleEx(fromAddress, address, module) == 0) {
      throw StateError(
        'The kosher_rust library is loaded, but its file was not found.',
      );
    }
    final length = getModuleFileName(module.value, name, capacity);
    if (length == 0) {
      throw StateError(
        'The kosher_rust library is loaded, but its file was not found.',
      );
    }
    return name.toDartString(length: length);
  } finally {
    calloc.free(module);
    calloc.free(name);
  }
}
