import init, {eval_expression} from './wasm/wasm.js';

const inputBox = document.getElementById("inputId");
const submitButton = document.getElementById("submitId");
const resultText = document.getElementById("result");

(async () => {
  await init();

  submitButton.onclick = function(){
    resultText.innerHTML = eval_expression(inputBox.value);
  };
})();
