const state = {};
const functionsMap = {};

const select = document.querySelector("select");
const [input1, input2] = document.querySelectorAll("input");
const button = document.querySelector("button");
const resultDisplay = document.querySelector("span");

select.onchange = function (e) {
  state.operation = e.target.value;
};
input1.onchange = function (e) {
  state.value1 = e.target.value;
};
input2.onchange = function (e) {
  state.value2 = e.target.value;
};
button.onclick = function () {
  const { operation, value1, value2 } = state;
  const result = functionsMap[operation](value1, value2);
  resultDisplay.innerText = result;
};

function populateSelect(choices) {
  select.innerHTML = "";
  choices.forEach((label) => {
    console.log(label);
    const newOption = document.createElement("option");
    newOption.innerText = label;
    select.appendChild(newOption);
  });
}

async function loadWASM() {
  const module = await WebAssembly.instantiateStreaming(
    fetch("calcplus.wasm"),
    {}
  );
  Object.assign(
    functionsMap,
    Object.fromEntries(
      Object.entries(module.instance.exports).filter(([_, val]) => {
        return typeof val === "function";
      })
    )
  );
  populateSelect(Object.keys(functionsMap));
}
loadWASM();
