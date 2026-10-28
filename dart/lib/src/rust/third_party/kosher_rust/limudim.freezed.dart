// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'limudim.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$PirkeiAvosUnit {

 int get field0;
/// Create a copy of PirkeiAvosUnit
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PirkeiAvosUnitCopyWith<PirkeiAvosUnit> get copyWith => _$PirkeiAvosUnitCopyWithImpl<PirkeiAvosUnit>(this as PirkeiAvosUnit, _$identity);



@override
bool operator ==(Object other) {
  final _this = this as PirkeiAvosUnit;
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PirkeiAvosUnit&&(identical(other.field0, _this.field0) || other.field0 == _this.field0));
}


@override
int get hashCode {
  final _this = this as PirkeiAvosUnit;
  return Object.hash(runtimeType,_this.field0);
}



}

/// @nodoc
abstract mixin class $PirkeiAvosUnitCopyWith<$Res>  {
  factory $PirkeiAvosUnitCopyWith(PirkeiAvosUnit value, $Res Function(PirkeiAvosUnit) _then) = _$PirkeiAvosUnitCopyWithImpl;
@useResult
$Res call({
 int field0
});




}
/// @nodoc
class _$PirkeiAvosUnitCopyWithImpl<$Res>
    implements $PirkeiAvosUnitCopyWith<$Res> {
  _$PirkeiAvosUnitCopyWithImpl(this._self, this._then);

  final PirkeiAvosUnit _self;
  final $Res Function(PirkeiAvosUnit) _then;

/// Create a copy of PirkeiAvosUnit
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [PirkeiAvosUnit].
extension PirkeiAvosUnitPatterns on PirkeiAvosUnit {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( PirkeiAvosUnit_Single value)?  single,TResult Function( PirkeiAvosUnit_Combined value)?  combined,required TResult orElse(),}){
final _that = this;
switch (_that) {
case PirkeiAvosUnit_Single() when single != null:
return single(_that);case PirkeiAvosUnit_Combined() when combined != null:
return combined(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( PirkeiAvosUnit_Single value)  single,required TResult Function( PirkeiAvosUnit_Combined value)  combined,}){
final _that = this;
switch (_that) {
case PirkeiAvosUnit_Single():
return single(_that);case PirkeiAvosUnit_Combined():
return combined(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( PirkeiAvosUnit_Single value)?  single,TResult? Function( PirkeiAvosUnit_Combined value)?  combined,}){
final _that = this;
switch (_that) {
case PirkeiAvosUnit_Single() when single != null:
return single(_that);case PirkeiAvosUnit_Combined() when combined != null:
return combined(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( int field0)?  single,TResult Function( int field0,  int field1)?  combined,required TResult orElse(),}) {final _that = this;
switch (_that) {
case PirkeiAvosUnit_Single() when single != null:
return single(_that.field0);case PirkeiAvosUnit_Combined() when combined != null:
return combined(_that.field0,_that.field1);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( int field0)  single,required TResult Function( int field0,  int field1)  combined,}) {final _that = this;
switch (_that) {
case PirkeiAvosUnit_Single():
return single(_that.field0);case PirkeiAvosUnit_Combined():
return combined(_that.field0,_that.field1);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( int field0)?  single,TResult? Function( int field0,  int field1)?  combined,}) {final _that = this;
switch (_that) {
case PirkeiAvosUnit_Single() when single != null:
return single(_that.field0);case PirkeiAvosUnit_Combined() when combined != null:
return combined(_that.field0,_that.field1);case _:
  return null;

}
}

}

/// @nodoc


class PirkeiAvosUnit_Single extends PirkeiAvosUnit {
  const PirkeiAvosUnit_Single(this.field0): super._();
  

@override final  int field0;

/// Create a copy of PirkeiAvosUnit
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PirkeiAvosUnit_SingleCopyWith<PirkeiAvosUnit_Single> get copyWith => _$PirkeiAvosUnit_SingleCopyWithImpl<PirkeiAvosUnit_Single>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is PirkeiAvosUnit_Single&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0);
}



}

/// @nodoc
abstract mixin class $PirkeiAvosUnit_SingleCopyWith<$Res> implements $PirkeiAvosUnitCopyWith<$Res> {
  factory $PirkeiAvosUnit_SingleCopyWith(PirkeiAvosUnit_Single value, $Res Function(PirkeiAvosUnit_Single) _then) = _$PirkeiAvosUnit_SingleCopyWithImpl;
@override @useResult
$Res call({
 int field0
});




}
/// @nodoc
class _$PirkeiAvosUnit_SingleCopyWithImpl<$Res>
    implements $PirkeiAvosUnit_SingleCopyWith<$Res> {
  _$PirkeiAvosUnit_SingleCopyWithImpl(this._self, this._then);

  final PirkeiAvosUnit_Single _self;
  final $Res Function(PirkeiAvosUnit_Single) _then;

/// Create a copy of PirkeiAvosUnit
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(PirkeiAvosUnit_Single(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc


class PirkeiAvosUnit_Combined extends PirkeiAvosUnit {
  const PirkeiAvosUnit_Combined(this.field0, this.field1): super._();
  

@override final  int field0;
 final  int field1;

/// Create a copy of PirkeiAvosUnit
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PirkeiAvosUnit_CombinedCopyWith<PirkeiAvosUnit_Combined> get copyWith => _$PirkeiAvosUnit_CombinedCopyWithImpl<PirkeiAvosUnit_Combined>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is PirkeiAvosUnit_Combined&&(identical(other.field0, field0) || other.field0 == field0)&&(identical(other.field1, field1) || other.field1 == field1));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0,field1);
}



}

/// @nodoc
abstract mixin class $PirkeiAvosUnit_CombinedCopyWith<$Res> implements $PirkeiAvosUnitCopyWith<$Res> {
  factory $PirkeiAvosUnit_CombinedCopyWith(PirkeiAvosUnit_Combined value, $Res Function(PirkeiAvosUnit_Combined) _then) = _$PirkeiAvosUnit_CombinedCopyWithImpl;
@override @useResult
$Res call({
 int field0, int field1
});




}
/// @nodoc
class _$PirkeiAvosUnit_CombinedCopyWithImpl<$Res>
    implements $PirkeiAvosUnit_CombinedCopyWith<$Res> {
  _$PirkeiAvosUnit_CombinedCopyWithImpl(this._self, this._then);

  final PirkeiAvosUnit_Combined _self;
  final $Res Function(PirkeiAvosUnit_Combined) _then;

/// Create a copy of PirkeiAvosUnit
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,Object? field1 = null,}) {
  return _then(PirkeiAvosUnit_Combined(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as int,null == field1 ? _self.field1 : field1 // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc
mixin _$TehillimUnit {





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is TehillimUnit);
}


@override
int get hashCode => runtimeType.hashCode;



}

/// @nodoc
class $TehillimUnitCopyWith<$Res>  {
$TehillimUnitCopyWith(TehillimUnit _, $Res Function(TehillimUnit) __);
}


/// Adds pattern-matching-related methods to [TehillimUnit].
extension TehillimUnitPatterns on TehillimUnit {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( TehillimUnit_Psalms value)?  psalms,TResult Function( TehillimUnit_PsalmVerses value)?  psalmVerses,required TResult orElse(),}){
final _that = this;
switch (_that) {
case TehillimUnit_Psalms() when psalms != null:
return psalms(_that);case TehillimUnit_PsalmVerses() when psalmVerses != null:
return psalmVerses(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( TehillimUnit_Psalms value)  psalms,required TResult Function( TehillimUnit_PsalmVerses value)  psalmVerses,}){
final _that = this;
switch (_that) {
case TehillimUnit_Psalms():
return psalms(_that);case TehillimUnit_PsalmVerses():
return psalmVerses(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( TehillimUnit_Psalms value)?  psalms,TResult? Function( TehillimUnit_PsalmVerses value)?  psalmVerses,}){
final _that = this;
switch (_that) {
case TehillimUnit_Psalms() when psalms != null:
return psalms(_that);case TehillimUnit_PsalmVerses() when psalmVerses != null:
return psalmVerses(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( int start,  int end)?  psalms,TResult Function( int psalm,  int startVerse,  int endVerse)?  psalmVerses,required TResult orElse(),}) {final _that = this;
switch (_that) {
case TehillimUnit_Psalms() when psalms != null:
return psalms(_that.start,_that.end);case TehillimUnit_PsalmVerses() when psalmVerses != null:
return psalmVerses(_that.psalm,_that.startVerse,_that.endVerse);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( int start,  int end)  psalms,required TResult Function( int psalm,  int startVerse,  int endVerse)  psalmVerses,}) {final _that = this;
switch (_that) {
case TehillimUnit_Psalms():
return psalms(_that.start,_that.end);case TehillimUnit_PsalmVerses():
return psalmVerses(_that.psalm,_that.startVerse,_that.endVerse);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( int start,  int end)?  psalms,TResult? Function( int psalm,  int startVerse,  int endVerse)?  psalmVerses,}) {final _that = this;
switch (_that) {
case TehillimUnit_Psalms() when psalms != null:
return psalms(_that.start,_that.end);case TehillimUnit_PsalmVerses() when psalmVerses != null:
return psalmVerses(_that.psalm,_that.startVerse,_that.endVerse);case _:
  return null;

}
}

}

/// @nodoc


class TehillimUnit_Psalms extends TehillimUnit {
  const TehillimUnit_Psalms({required this.start, required this.end}): super._();
  

/// The first chapter.
 final  int start;
/// The last chapter.
 final  int end;

/// Create a copy of TehillimUnit
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TehillimUnit_PsalmsCopyWith<TehillimUnit_Psalms> get copyWith => _$TehillimUnit_PsalmsCopyWithImpl<TehillimUnit_Psalms>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is TehillimUnit_Psalms&&(identical(other.start, start) || other.start == start)&&(identical(other.end, end) || other.end == end));
}


@override
int get hashCode {
    return Object.hash(runtimeType,start,end);
}



}

/// @nodoc
abstract mixin class $TehillimUnit_PsalmsCopyWith<$Res> implements $TehillimUnitCopyWith<$Res> {
  factory $TehillimUnit_PsalmsCopyWith(TehillimUnit_Psalms value, $Res Function(TehillimUnit_Psalms) _then) = _$TehillimUnit_PsalmsCopyWithImpl;
@useResult
$Res call({
 int start, int end
});




}
/// @nodoc
class _$TehillimUnit_PsalmsCopyWithImpl<$Res>
    implements $TehillimUnit_PsalmsCopyWith<$Res> {
  _$TehillimUnit_PsalmsCopyWithImpl(this._self, this._then);

  final TehillimUnit_Psalms _self;
  final $Res Function(TehillimUnit_Psalms) _then;

/// Create a copy of TehillimUnit
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? start = null,Object? end = null,}) {
  return _then(TehillimUnit_Psalms(
start: null == start ? _self.start : start // ignore: cast_nullable_to_non_nullable
as int,end: null == end ? _self.end : end // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc


class TehillimUnit_PsalmVerses extends TehillimUnit {
  const TehillimUnit_PsalmVerses({required this.psalm, required this.startVerse, required this.endVerse}): super._();
  

/// The chapter.
 final  int psalm;
/// The first verse.
 final  int startVerse;
/// The last verse.
 final  int endVerse;

/// Create a copy of TehillimUnit
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TehillimUnit_PsalmVersesCopyWith<TehillimUnit_PsalmVerses> get copyWith => _$TehillimUnit_PsalmVersesCopyWithImpl<TehillimUnit_PsalmVerses>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is TehillimUnit_PsalmVerses&&(identical(other.psalm, psalm) || other.psalm == psalm)&&(identical(other.startVerse, startVerse) || other.startVerse == startVerse)&&(identical(other.endVerse, endVerse) || other.endVerse == endVerse));
}


@override
int get hashCode {
    return Object.hash(runtimeType,psalm,startVerse,endVerse);
}



}

/// @nodoc
abstract mixin class $TehillimUnit_PsalmVersesCopyWith<$Res> implements $TehillimUnitCopyWith<$Res> {
  factory $TehillimUnit_PsalmVersesCopyWith(TehillimUnit_PsalmVerses value, $Res Function(TehillimUnit_PsalmVerses) _then) = _$TehillimUnit_PsalmVersesCopyWithImpl;
@useResult
$Res call({
 int psalm, int startVerse, int endVerse
});




}
/// @nodoc
class _$TehillimUnit_PsalmVersesCopyWithImpl<$Res>
    implements $TehillimUnit_PsalmVersesCopyWith<$Res> {
  _$TehillimUnit_PsalmVersesCopyWithImpl(this._self, this._then);

  final TehillimUnit_PsalmVerses _self;
  final $Res Function(TehillimUnit_PsalmVerses) _then;

/// Create a copy of TehillimUnit
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? psalm = null,Object? startVerse = null,Object? endVerse = null,}) {
  return _then(TehillimUnit_PsalmVerses(
psalm: null == psalm ? _self.psalm : psalm // ignore: cast_nullable_to_non_nullable
as int,startVerse: null == startVerse ? _self.startVerse : startVerse // ignore: cast_nullable_to_non_nullable
as int,endVerse: null == endVerse ? _self.endVerse : endVerse // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

// dart format on
