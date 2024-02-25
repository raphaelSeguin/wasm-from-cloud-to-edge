package main

import (
    "fmt"
    "io/ioutil"

    wasm "github.com/wasmerio/wasmer-go/wasmer"
)

func main() {
    wasmBytes, _ := ioutil.ReadFile("./calc.wasm")

    engine := wasm.NewEngine()
    store := wasm.NewStore(engine)

    module, _ := wasm.NewModule(store, wasmBytes)

    importObject := wasm.NewImportObject()
    instance, _ := wasm.NewInstance(module, importObject)

    add, _ := instance.Exports.GetFunction("add")
		sub, _ := instance.Exports.GetFunction("sub")
		mul, _ := instance.Exports.GetFunction("mul")
		div, _ := instance.Exports.GetFunction("div")
    result, _ := add(2, 8)
		resultsub, _ := sub(12, 3)
		resultmul, _ := mul(9, 12)
		resultdiv, _ := div(777, 11)
    fmt.Println(result)
		fmt.Println(resultsub)
		fmt.Println(resultmul)
		fmt.Println(resultdiv)
}