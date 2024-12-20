import 'package:collection/collection.dart';

class UnitSet {
  final Map<Unit, int> units;

  const UnitSet(this.units);

  const UnitSet.empty() : units = const {};

  UnitSet operator *(UnitSet other) {
    final newUnits = Map<Unit, int>.from(units);
    for (final entry in other.units.entries) {
      newUnits.update(entry.key, (value) => value + entry.value,
          ifAbsent: () => entry.value);
      if (newUnits[entry.key] == 0) {
        newUnits.remove(entry.key);
      }
    }
    return UnitSet(newUnits);
  }

  UnitSet operator /(UnitSet other) {
    final newUnits = Map<Unit, int>.from(units);
    for (final entry in other.units.entries) {
      newUnits.update(entry.key, (value) => value - entry.value,
          ifAbsent: () => -entry.value);
      if (newUnits[entry.key] == 0) {
        newUnits.remove(entry.key);
      }
    }
    return UnitSet(newUnits);
  }

  UnitSet pow(int power) {
    final newUnits = Map<Unit, int>.from(units);
    for (final entry in newUnits.entries) {
      newUnits[entry.key] = entry.value * power;
    }
    return UnitSet(newUnits);
  }

  @override
  bool operator ==(Object other) =>
      other is UnitSet && const MapEquality().equals(units, other.units);

  @override
  int get hashCode => const MapEquality().hash(units);

  @override
  String toString() {
    return 'UnitSet($units)';
  }
}

class Unit {
  final String name;

  const Unit(this.name);

  // Do not override == and hashCode so that we can use reference equality.
}
