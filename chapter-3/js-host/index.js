console.log("YOLO WASM !");

WebAssembly.instantiateStreaming(fetch("calc.wasm"), {}).then((obj) => {
  console.log(obj.instance.exports.add(12, 23));
  console.log(obj.instance.exports.mul(3, 7));
  console.log(obj.instance.exports.sub(1234, 23));
  console.log(obj.instance.exports.div(1234, 32));
  console.log(obj);
});
