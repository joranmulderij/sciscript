class UnitSet {
  final Map<Unit, int> units;

  const UnitSet(this.units);

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
}

class Unit {
  final String name;

  const Unit(this.name);
}
