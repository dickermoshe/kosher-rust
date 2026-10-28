// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'zmanim.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$AstronomicalCalculator {





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AstronomicalCalculator);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'AstronomicalCalculator()';
}


}

/// @nodoc
class $AstronomicalCalculatorCopyWith<$Res>  {
$AstronomicalCalculatorCopyWith(AstronomicalCalculator _, $Res Function(AstronomicalCalculator) __);
}


/// Adds pattern-matching-related methods to [AstronomicalCalculator].
extension AstronomicalCalculatorPatterns on AstronomicalCalculator {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AstronomicalCalculator_Noaa value)?  noaa,TResult Function( AstronomicalCalculator_Meeus value)?  meeus,TResult Function( AstronomicalCalculator_Spa value)?  spa,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AstronomicalCalculator_Noaa() when noaa != null:
return noaa(_that);case AstronomicalCalculator_Meeus() when meeus != null:
return meeus(_that);case AstronomicalCalculator_Spa() when spa != null:
return spa(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AstronomicalCalculator_Noaa value)  noaa,required TResult Function( AstronomicalCalculator_Meeus value)  meeus,required TResult Function( AstronomicalCalculator_Spa value)  spa,}){
final _that = this;
switch (_that) {
case AstronomicalCalculator_Noaa():
return noaa(_that);case AstronomicalCalculator_Meeus():
return meeus(_that);case AstronomicalCalculator_Spa():
return spa(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AstronomicalCalculator_Noaa value)?  noaa,TResult? Function( AstronomicalCalculator_Meeus value)?  meeus,TResult? Function( AstronomicalCalculator_Spa value)?  spa,}){
final _that = this;
switch (_that) {
case AstronomicalCalculator_Noaa() when noaa != null:
return noaa(_that);case AstronomicalCalculator_Meeus() when meeus != null:
return meeus(_that);case AstronomicalCalculator_Spa() when spa != null:
return spa(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  noaa,TResult Function( bool applyDeltaT)?  meeus,TResult Function( bool applyDeltaT)?  spa,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AstronomicalCalculator_Noaa() when noaa != null:
return noaa();case AstronomicalCalculator_Meeus() when meeus != null:
return meeus(_that.applyDeltaT);case AstronomicalCalculator_Spa() when spa != null:
return spa(_that.applyDeltaT);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  noaa,required TResult Function( bool applyDeltaT)  meeus,required TResult Function( bool applyDeltaT)  spa,}) {final _that = this;
switch (_that) {
case AstronomicalCalculator_Noaa():
return noaa();case AstronomicalCalculator_Meeus():
return meeus(_that.applyDeltaT);case AstronomicalCalculator_Spa():
return spa(_that.applyDeltaT);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  noaa,TResult? Function( bool applyDeltaT)?  meeus,TResult? Function( bool applyDeltaT)?  spa,}) {final _that = this;
switch (_that) {
case AstronomicalCalculator_Noaa() when noaa != null:
return noaa();case AstronomicalCalculator_Meeus() when meeus != null:
return meeus(_that.applyDeltaT);case AstronomicalCalculator_Spa() when spa != null:
return spa(_that.applyDeltaT);case _:
  return null;

}
}

}

/// @nodoc


class AstronomicalCalculator_Noaa extends AstronomicalCalculator {
  const AstronomicalCalculator_Noaa(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AstronomicalCalculator_Noaa);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'AstronomicalCalculator.noaa()';
}


}




/// @nodoc


class AstronomicalCalculator_Meeus extends AstronomicalCalculator {
  const AstronomicalCalculator_Meeus({required this.applyDeltaT}): super._();
  

/// Whether to apply the ΔT (TT − UT) correction.
 final  bool applyDeltaT;

/// Create a copy of AstronomicalCalculator
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AstronomicalCalculator_MeeusCopyWith<AstronomicalCalculator_Meeus> get copyWith => _$AstronomicalCalculator_MeeusCopyWithImpl<AstronomicalCalculator_Meeus>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AstronomicalCalculator_Meeus&&(identical(other.applyDeltaT, applyDeltaT) || other.applyDeltaT == applyDeltaT));
}


@override
int get hashCode {
    return Object.hash(runtimeType,applyDeltaT);
}

@override
String toString() {
    return 'AstronomicalCalculator.meeus(applyDeltaT: $applyDeltaT)';
}


}

/// @nodoc
abstract mixin class $AstronomicalCalculator_MeeusCopyWith<$Res> implements $AstronomicalCalculatorCopyWith<$Res> {
  factory $AstronomicalCalculator_MeeusCopyWith(AstronomicalCalculator_Meeus value, $Res Function(AstronomicalCalculator_Meeus) _then) = _$AstronomicalCalculator_MeeusCopyWithImpl;
@useResult
$Res call({
 bool applyDeltaT
});




}
/// @nodoc
class _$AstronomicalCalculator_MeeusCopyWithImpl<$Res>
    implements $AstronomicalCalculator_MeeusCopyWith<$Res> {
  _$AstronomicalCalculator_MeeusCopyWithImpl(this._self, this._then);

  final AstronomicalCalculator_Meeus _self;
  final $Res Function(AstronomicalCalculator_Meeus) _then;

/// Create a copy of AstronomicalCalculator
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? applyDeltaT = null,}) {
  return _then(AstronomicalCalculator_Meeus(
applyDeltaT: null == applyDeltaT ? _self.applyDeltaT : applyDeltaT // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc


class AstronomicalCalculator_Spa extends AstronomicalCalculator {
  const AstronomicalCalculator_Spa({required this.applyDeltaT}): super._();
  

/// Whether to apply the ΔT (TT − UT) correction.
 final  bool applyDeltaT;

/// Create a copy of AstronomicalCalculator
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AstronomicalCalculator_SpaCopyWith<AstronomicalCalculator_Spa> get copyWith => _$AstronomicalCalculator_SpaCopyWithImpl<AstronomicalCalculator_Spa>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AstronomicalCalculator_Spa&&(identical(other.applyDeltaT, applyDeltaT) || other.applyDeltaT == applyDeltaT));
}


@override
int get hashCode {
    return Object.hash(runtimeType,applyDeltaT);
}

@override
String toString() {
    return 'AstronomicalCalculator.spa(applyDeltaT: $applyDeltaT)';
}


}

/// @nodoc
abstract mixin class $AstronomicalCalculator_SpaCopyWith<$Res> implements $AstronomicalCalculatorCopyWith<$Res> {
  factory $AstronomicalCalculator_SpaCopyWith(AstronomicalCalculator_Spa value, $Res Function(AstronomicalCalculator_Spa) _then) = _$AstronomicalCalculator_SpaCopyWithImpl;
@useResult
$Res call({
 bool applyDeltaT
});




}
/// @nodoc
class _$AstronomicalCalculator_SpaCopyWithImpl<$Res>
    implements $AstronomicalCalculator_SpaCopyWith<$Res> {
  _$AstronomicalCalculator_SpaCopyWithImpl(this._self, this._then);

  final AstronomicalCalculator_Spa _self;
  final $Res Function(AstronomicalCalculator_Spa) _then;

/// Create a copy of AstronomicalCalculator
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? applyDeltaT = null,}) {
  return _then(AstronomicalCalculator_Spa(
applyDeltaT: null == applyDeltaT ? _self.applyDeltaT : applyDeltaT // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

// dart format on
