<template>
  <div>
    <h1>Renderer Test</h1>
  </div>
  <GameRenderer ref="gameRenderer" :assetStorage="assetStorage!" style="width: 500px; height: 500px; aspect-ratio: 1;"/>
  <button @click=""></button>
</template>

<script setup lang="ts">
import GameRenderer from "../components/GameRenderer.vue"
import { MemoryAssetStorage } from "../asset_storage/MemoryAssetStorage";
import test_zip_url from "./test.zip";
import { onBeforeMount, onMounted, ref } from "vue"
import { SceneElement, SceneUpdate, UpdateType } from "../.gen/proto/scene_update"
import { DrawBundle } from "../.gen/proto/draw_bundle";
import { VertexAttributes } from "../.gen/proto/vertex_attributes";
import { UniformAttributes } from "../.gen/proto/uniform_attributes";
import { FloatArray } from "../.gen/proto/types";
import { mat4, vec3 } from 'gl-matrix';

const assetStorage = ref<MemoryAssetStorage>();
const gameRenderer = ref<InstanceType<typeof GameRenderer> | null>(null);

onBeforeMount(async () => {
  assetStorage.value = new MemoryAssetStorage();
  await assetStorage.value.appendFromZipRemote(test_zip_url);
});

onMounted(() => {
  requestAnimationFrame(draw);
  gameRenderer?.value?.resolution(500, 500);
})

async function draw() {
  if (!gameRenderer) return;
  const renderer = gameRenderer?.value;
  const update = SceneUpdate.create();
  update.type = UpdateType.Full;

  var seconds = new Date().getTime() / 1000;

  const projection = mat4.perspective(mat4.create(), 45 * Math.PI / 180, 1.0, 0.1, 1000); 
  const projectionFloatArray = FloatArray.create();
  projectionFloatArray.values = Array.from(projection);

  const phase = (2 * Math.PI / 5.0) * seconds;
  const view = mat4.lookAt(mat4.create(),
    vec3.fromValues(8*Math.sin(phase), 0, -8*Math.cos(phase)),
    vec3.fromValues(0, 0, 0),
    vec3.fromValues(0, 1, 0)
  );
  const viewFloatArray = FloatArray.create();
  viewFloatArray.values = Array.from(view);

  const identityMat = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
  ];
  const identityMapFloatArray = FloatArray.create();
  identityMapFloatArray.values = identityMat;

  const draw_bundle = DrawBundle.create();
  draw_bundle.billboard = false;
  draw_bundle.layer = 1;
  draw_bundle.meshAsset = "/assets/cube.json";
  draw_bundle.pixelShaderAsset = "/assets/diffuse_shader.ps.glsl",
  draw_bundle.vertexShaderAsset = "/assets/diffuse_shader.vs.glsl",
  draw_bundle.vertexAttributes = VertexAttributes.create();
  draw_bundle.vertexAttributes.vertices = "aPos";
  draw_bundle.vertexAttributes.namedBuffers = {
    "normals": "aNormal"
  };
  draw_bundle.uniformAttributes = UniformAttributes.create();
  draw_bundle.uniformAttributes.mat4 = {
    "model": identityMapFloatArray,
    "view": viewFloatArray,
    "projection": projectionFloatArray
  };

  const scene_element = SceneElement.create();
  scene_element.drawBundle = draw_bundle;
  scene_element.id = "test_element";

  update.elements.push(scene_element);
  
  await renderer?.update(update);
  await renderer?.render();

  requestAnimationFrame(draw);
}

</script>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
