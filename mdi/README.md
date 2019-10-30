# Material Design Icons for Flutter

Material Design Icons generated using `mdi/fonts` SVG font provided by
[materialdesignicons.com](https://materialdesignicons.com).

### Install

```yaml
mdi: 0.2.2
```

### Usage

```dart
import 'package:mdi/mdi.dart';

class AccessPointButton extends StatelessWidget {
  Widget build(BuildContext context) {
    return IconButton(
      icon: Icon(Mdi.accessPoint),
     );
  }
}
```

### Naming

The icon names provided via `Mdi` are camel-cased variants of the original name.

##### Exceptions:

1. null -> nullIcon
2. switch -> switchIcon
3. sync -> syncIcon
4. factory -> factoryIcon
