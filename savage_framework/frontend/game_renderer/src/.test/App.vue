<template>
  <div>
    <h1>Renderer Test</h1>
  </div>
  <GameRenderer ref="gameRenderer" :assetStorage="assetStorage!" style="width: 100%; height: 100%; aspect-ratio: 1;"/>
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
import { Float32Array } from "../.gen/proto/types";
import { mat4, vec3 } from 'gl-matrix';

const assetStorage = ref<MemoryAssetStorage>();
const gameRenderer = ref<InstanceType<typeof GameRenderer> | null>(null);

onBeforeMount(async () => {
  assetStorage.value = new MemoryAssetStorage();
  await assetStorage.value.appendFromZipRemote(test_zip_url);
});

onMounted(() => {
  requestAnimationFrame(draw);
  gameRenderer?.value?.resolution(1024, 1024);
})

async function draw_asset(mesh_asset: string, translation: vec3, time: number): Promise<SceneElement> {
  var seconds = time;

  const identityMat = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
  ];
  const identityMapFloatArray = Float32Array.create();
  identityMapFloatArray.values = identityMat;

  const phase = (2 * Math.PI / 5.0) * seconds;
  let model = mat4.translate(mat4.create(), mat4.create(), translation);
  model = mat4.rotate(model, model, 2 * phase, vec3.fromValues(0, 1, 0));
  model = mat4.rotate(model, model, phase, vec3.fromValues(1, 1, 0));
  const modelFloatArray = Float32Array.create();
  modelFloatArray.values = Array.from(model);

  const draw_bundle = DrawBundle.create();
  draw_bundle.billboard = false;
  draw_bundle.layer = 1;
  draw_bundle.meshAsset = mesh_asset;
  draw_bundle.pixelShaderAsset = "/assets/diffuse_shader.ps.glsl",
  draw_bundle.vertexShaderAsset = "/assets/diffuse_shader.vs.glsl",
  draw_bundle.vertexAttributes = VertexAttributes.create();
  draw_bundle.vertexAttributes.vertices = "aPos";
  draw_bundle.vertexAttributes.namedBuffers = {
    "normals": "aNormal"
  };
  draw_bundle.uniformAttributes = UniformAttributes.create();
  draw_bundle.uniformAttributes.mat4 = {
    "model": modelFloatArray
  };

  const scene_element = SceneElement.create();
  scene_element.drawBundle = draw_bundle;
  scene_element.id = "test_element";

  return scene_element;
}

async function draw() {
  if (!gameRenderer) return;
  const renderer = gameRenderer?.value;
  const update = SceneUpdate.create();
  update.type = UpdateType.Full;

  var seconds = new Date().getTime() / 1000;
  const projection = mat4.perspective(mat4.create(), 45 * Math.PI / 180, 1.0, 0.1, 1000); 
  const projectionFloatArray = Float32Array.create();
  projectionFloatArray.values = Array.from(projection);
  
  const view = mat4.translate(mat4.create(), mat4.create(), vec3.fromValues(0, 0, -10));

  const viewFloatArray = Float32Array.create();
  viewFloatArray.values = Array.from(view);

  const sharedAttrs = UniformAttributes.create();
  sharedAttrs.mat4 = {
    'view': viewFloatArray,
    'projection': projectionFloatArray
  };

  update.sharedAttributes = sharedAttrs;

  update.elements.push(await draw_asset('/assets/sphere.json', vec3.fromValues(2, 2, 0), seconds));
  update.elements.push(await draw_asset('/assets/torus.json', vec3.fromValues(2, -2, 0), seconds));
  update.elements.push(await draw_asset('/assets/monkey.json', vec3.fromValues(-2, -2, 0), seconds));
  update.elements.push(await draw_asset('/assets/cube.json', vec3.fromValues(-2, 2, 0), seconds));

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
