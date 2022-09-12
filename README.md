# Material Design Icons for Flutter

Material Design Icons generated using `@mdi/util` provided by [materialdesignicons.com](https://materialdesignicons.com).

### Installation

Add this to your package's pubspec.yaml file:

```yaml
dependencies:
  mdi:
    git:
      url: https://github.com/bleonard252/mdi-dart
      path: mdi
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

### Contributing

If you want to contribute to this project, you may easily create issues and send PRs. Please take note that your code contributions will be applicable under MIT license unless specified otherwise.
