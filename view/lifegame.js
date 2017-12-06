const rust = require('rustify')

const wasm = rust('../src/render.rs')

WebAssembly.instantiate(wasm, {})
  .then(function (res) {
    const update = res.instance.exports.update
    const create = res.instance.exports.create
    const initialize = res.instance.exports.initialize
    const getElement = res.instance.exports.get_element

    const len = 25
    let buf = create()
    for (let i = 0; i < len; i++) {
      console.log(i, getElement(i, buf, len))
    }
    //initialize(buf, len)
    update(buf, len)
    for (let i = 0; i < len; i++) {
      console.log(i, getElement(i, buf, len))
    }
  }).catch(function (e) {
    console.error('Creating WASM module failed', e)
  })
