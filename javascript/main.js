(function() {
    var TARGET;
    var AFTER_POINT;
    var INPUT_BOX_SIZE = 3;

    var sourceElm = document.querySelector("#source");
    var resultElm = document.querySelector("#result");
    var calcElm = document.querySelector("#calc");
    var afterPointElm = document.querySelector("#afterPoint");
    var targetElm = document.querySelector("#target");
    calcElm.addEventListener("click", calc);

    function calc() {
    var source = sourceElm.value.split(",").map(e => parseFloat(e));
    AFTER_POINT = parseInt(afterPointElm.value);
    TARGET = parseInt(targetElm.value);
    var sourceModified = [];
    var variants = [];

    for (var i = 0; i < source.length; i++) {
        for (var n = 0; n < source.length; n++) {
        try {
            var twoDigit = `${source[i]}${source[n]}`;
            var d = parseFloat(twoDigit);
            sourceModified.push(d);
        } catch (e) {}
        }
        for (var p = 1; p <= AFTER_POINT; p++) {
        var variant = new Variants(source, p);
        for (let v of variant) {
            try {
            var withPoint = `${source[i]}.${v.join('')}`;
            var d = parseFloat(withPoint);
            sourceModified.push(d);
            } catch (e) {}
        }
        }
    }

    sourceModified = source
        .concat(sourceModified)
        .filter(item => item < TARGET)
        .filter((item, i, arr) => arr.indexOf(item) === i);   
    
    
    var sourceVariants = new Variants(sourceModified, INPUT_BOX_SIZE);
    
    console.log("Variants items", sourceModified.length);
    console.log("Variants count", sourceVariants.variantsSize);
    
    for ( let variant of sourceVariants) {
        let sum = variant.reduce((s, c) => s + c, 0);
        if (sum == TARGET) {
        variants.push(variant);
        }
    }
    

    var result = variants
        .map(r => r.sort((a, b) => a - b).join(","))
        .filter((r, i, arr) => arr.indexOf(r) === i)
        .map(r => r.split(","));

    printResult(result);
    }

    function printResult(result) {
    resultElm.innerHTML = "";
    var printResult = `<span>Result: ${result.length} values</span>`;
    for (var i = 0; i < result.length; i++) {
        printResult += `<li>${result[i].join(" + ")} = ${TARGET}</li>`;
    }
    resultElm.innerHTML = printResult;
    }



    class Variants {
        constructor(arr, basis) {
        this.arr = arr;
        this.basis = basis;
        this.variantsSize = Math.pow(arr.length, basis);
        this.variantsCount = 0;
        this.convert = this.converter(10, arr.length);
        }

        [Symbol.iterator]() {
        return {
            next: () => {
                let result = this.convert(this.variantsCount++)
                    .map(k => this.arr[k]);
                return {
                    value: result,
                    done: this.variantsCount > this.variantsSize
                }
            }
        }
        }

        converter(from, to) {
        return (num) => {
        var cur = num;
        var result = new Array(this.basis).fill(0);
        var count = this.basis - 1;
        while (cur) {
            var res = cur % to;
            cur = Math.floor(cur / to);
            result[count--] = res;
        }
        return result;
        }
    }
    }    
})();