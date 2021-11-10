import init, {eval_expression} from './evaluate.js';

const inputBox = document.getElementById("inputId");
const resultText = document.getElementById("result");

(async () => {
  await init();

  inputBox.oninput = function() {
    resultText.innerHTML = eval_expression(inputBox.value);
  }
})();
