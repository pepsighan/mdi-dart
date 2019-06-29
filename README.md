# Material Design Icons for Flutter

Material Design Icons generated using `mdi/fonts` SVG font provided by
[materialdesignicons.com](https://materialdesignicons.com).

### Install

```yaml
mdi: 0.1.0
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

The icon names provided via `Mdi` are camelCased variants of the original name.

##### Exceptions:

1. null -> nullIcon
2. switch -> switchIcon
3. sync -> syncIcon
4. factory -> factoryIcon

### Want to help?

If you find the icons are outdated or there are bugs to be fixed, just submit a PR.

Also, to generate icon glue-code locally you will be required to
[install Rust & Cargo](https://www.rust-lang.org/tools/install).
If you already have Rust, run the following on terminal:

```
$ cargo run
```
