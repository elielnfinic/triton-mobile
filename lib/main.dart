import 'dart:convert';
import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class TritonResult {
  TritonResult(this.output, this.proof);

  final int output;
  final List<dynamic> proof;
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'ZKapp'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int number = 0;
  String text_number = "";
  bool show_animation = false;
  String calculating_text = "";
  String result_text = "";
  String elapsed_time = "";
  String proof_text = "";

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: Text(widget.title),
        ),
        body: Container(
          margin: EdgeInsets.all(20),
          child: Column(
            children: [
              TextField(
                decoration: InputDecoration(labelText: 'Enter your number'),
                keyboardType: TextInputType.number,
                onChanged: (value) {
                  setState(() {
                    text_number = value;
                  });
                },
              ),
              SizedBox(height: 20),
              Row(
                children: [
                  ElevatedButton(
                      onPressed: () async {
                        var stop_watch = Stopwatch()..start();
                        setState(() {
                          show_animation = true;
                          calculating_text = "Calculating...";
                        });
                        try {
                          number = int.parse(text_number);
                        } catch (ex) {
                          setState(() {
                            show_animation = false;
                            calculating_text = "";
                            result_text = "Please enter a valid number";
                          });
                          return;
                        }
                        String result = await compute(get_res_string, number);
                        stop_watch.stop();
                        final elapsedTime = stop_watch.elapsed;
                        setState(() {
                          elapsed_time =
                              "Elapsed time: ${elapsedTime.inMilliseconds} ms";
                        });
                        final json_decode = json.decode(result);
                        TritonResult tritonResult = TritonResult(
                            json_decode['output'], json_decode['proof']);
                        proof_text = tritonResult.proof.toString();

                        setState(() {
                          show_animation = false;
                          calculating_text = "";
                          result_text = tritonResult.output
                              .toString(); //"Factorial of $number is $result";
                        });
                      },
                      child: Text("Calculate"))
                ],
              ),
              show_animation ? CircularProgressIndicator() : Container(),
              calculating_text != "" ? Text(calculating_text) : Container(),
              result_text != "" ? Text(result_text) : Container(),
              elapsed_time != "" ? Text(elapsed_time) : Container(),
              Expanded(
                child: ListView(
                  children: [
                    ExpansionTile(
                      title: Text("Show TritonVM proof"),
                      children: [
                        // scrollable area
                        Container(
                          height: 200, // Set a fixed height for the scrollable area
                          child: SingleChildScrollView(
                            scrollDirection: Axis.vertical,
                            child: Text(proof_text),
                          ),
                        )
                      ],
                    ),
                  ],
                ),
              )
            ],
          ),
        ));
  }
}

Future<int> calculateFibonacci(int number) async {
  final DynamicLibrary nativeAddLib = Platform.isAndroid
      ? DynamicLibrary.open("libzknative.so")
      : DynamicLibrary.process();

  final int Function(int) calculateFibonacci = nativeAddLib
      .lookup<NativeFunction<Uint64 Function(Uint64)>>("run_triton")
      .asFunction();

  return calculateFibonacci(number);
}

Future<String> get_res_string(int number) async {
  final DynamicLibrary nativeAddLib = Platform.isAndroid
      ? DynamicLibrary.open("libzknative.so")
      : DynamicLibrary.process();

  final Pointer<Utf8> Function(int number) runTritonWithMeta = nativeAddLib
      .lookup<NativeFunction<Pointer<Utf8> Function(Uint64)>>(
          'run_triton_with_meta')
      .asFunction();

  final res = runTritonWithMeta(number);
  return res.toDartString();
}
