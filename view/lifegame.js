const rust = require('rustify')

const wasm = rust('../src/render.rs')

WebAssembly.instantiate(wasm, {})
  .then(function (res) {
    const update = res.instance.exports.update
    const create = res.instance.exports.create
    const initialize = res.instance.exports.initialize
    const getElement = res.instance.exports.get_element

    const row = 200
    const col = 200
    const len = row * col
    let buf = create()
    //for (let i = 0; i < len; i++) {
    //  console.log(i, getElement(i, buf, len))
    //}
    initialize(buf, len)
    update(buf, len)
    //for (let i = 0; i < len; i++) {
    //  console.log(i, getElement(i, buf, len))
    //}

    // rendering
    let canvas = document.getElementById("canvas")
    //canvas.width = window.innerWidth
    //canvas.height = window.innerHeight
    canvas.width = row
    canvas.height = col
    let ctx = canvas.getContext("2d")
    let imageData = ctx.createImageData(canvas.width, canvas.height)
    for (let i = 0; i < imageData.data.length; i += 4) {
      if (getElement(i/4, buf, len) == 0) {
        imageData.data[i] = 0
        imageData.data[i+1] = 0
        imageData.data[i+2] = 0
        imageData.data[i+3] = 255
      }
    }
    ctx.putImageData(imageData, 0, 0)

  }).catch(function (e) {
    console.error('Creating WASM module failed', e)
  })
