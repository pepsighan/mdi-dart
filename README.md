# Material Design Icons for Flutter

![pub version](https://badgen.net/pub/v/mdi) ![likes](https://badgen.net/pub/likes/mdi) ![sdk version](https://badgen.net/pub/sdk-version/mdi) ![platform](https://badgen.net/pub/flutter-platform/mdi) ![license](https://badgen.net/pub/license/mdi)

Material Design Icons generated using `@mdi/util` provided by [materialdesignicons.com](https://materialdesignicons.com).

### Installation
Add this to your package's pubspec.yaml file:

```yaml
dependencies:
  mdi: ^3.0.0
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
