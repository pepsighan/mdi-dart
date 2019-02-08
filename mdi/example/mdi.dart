import 'package:flutter/material.dart';
import 'package:mdi/mdi.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'MyApp',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: Column(
        children: <Widget>[
          // Use the camel-cased form of icon name.
          Icon(Mdi.bell),
          Text('Ring'),
        ],
      ),
    );
  }
}
