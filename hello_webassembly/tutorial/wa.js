// let add;
// 
// function loadWasm(filename) {
    // return fetch(filename)
    // .then(response => response.arrayBuffer())
    // .then(bytes => WebAssembly.instantiate(bytes))
    // .then(results => {
        // instance = results.instance;
        // document.getElementById('container').textContent = instance.exports.add();
    // }).catch(console.error);
// };
// 
// loadWasm('add.wasm')
// .then(instance => {
    // add = instance.exports._Z3addii;
// });
// 

(async() => {
    const codePromise = fetch('add.wasm')
    const module = await WebAssembly.instantiateStreaming(codePromise)
    console.log(module)
})
