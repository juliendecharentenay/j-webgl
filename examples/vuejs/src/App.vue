<template>
  <div>
    <canvas class="fixed inset-0 w-full h-full"></canvas>
    <ErrorElement 
      :error="error" @dismiss="() => {error = null;}" 
      v-if="error !== null" />
  </div>
</template>
<script setup>
import { ref, } from 'vue';

const error = ref(null);
const on_error = (o) => {error.value = o;};
const catcher = (label, f, d = null) => {
  try {
    return f();
  } catch (e) {
    on_error({msg: `Error in ${label}`, e});
  }
  return d;
};

const wasm_ = ref(null);
const renderer_ = ref(null);
const camera_ = ref(null);
import("./pkg")
.then(wasm => {
  const canvas = document.querySelector("canvas");
  let renderer = new wasm.MyRenderer(canvas);

  { const i = wasm.Initializer.new();
    renderer = renderer.with_initializer("background", i);
  }
  { const i = wasm.CubeWithNormals.new();
    renderer = renderer.with_cube_with_normals("cube", i);
  }
  /*
  { const i = wasm.Cube.new();
    renderer = renderer.with_cube("cube", i);
  }
  { renderer = renderer.without("cube");
  }
  */

  camera_.value = new wasm.Basic(canvas.clientWidth, canvas.clientHeight);
  renderer.render(camera_.value);

  wasm_.value = wasm;
  renderer_.value = renderer;

  requestAnimationFrame(() => {render();});
})
.catch(e => {on_error({msg: "Error importing wasm", e});});

const render = () => {
  catcher("render", () => {
    if (renderer_.value !== null && camera_.value !== null) {
      renderer_.value.render(camera_.value);
    }
    // requestAnimationFrame(() => {render();});
  });
};

window.addEventListener('resize', () => {
  catcher("resize", () => {
    if (renderer_.value !== null && camera_.value !== null) {
      console.log("resize");
      renderer_.value.resize(); render();
    }
  });
});

import ErrorElement from "./components/ErrorElement.vue";
</script>
