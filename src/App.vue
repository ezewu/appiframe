<template>
  <div class="container">
    <div class="header"><button id="toggleFullscreen">切换全屏/窗口模式</button> <button id="openFile">打开文件</button></div>

    <div class="content">
      <div class="center">
        <img :src="imageSrc" alt="" srcset="" />
      </div>
      <div class="right">
        <iframe :src="iframeSrc"></iframe>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref } from "vue";

import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { desktopDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

window.addEventListener("DOMContentLoaded", function () {
  setTimeout(() => {
    invoke("close_splashscreen");
  }, 1000);
});

const imageSrc = ref("");
const iframeSrc = ref("");
onMounted(() => {
  // 全屏切换
  document.getElementById("toggleFullscreen").addEventListener("click", async () => {
    await appWindow.setFullscreen(!(await appWindow.isFullscreen()));
  });

  document.getElementById("openFile").addEventListener("click", async () => {
    let files = await open({ directory: true, multiple: true });

    invoke("copy_folder", {
      src: files[0],
      dest: "C:\\11",
    })
      .then(() => {
        console.log("文件夹成功复制");
      })
      .catch((err) => {
        console.error("复制文件夹过程中发生错误：", err);
      });
  });

  desktopDir().then((res) => {
    console.log(res);
  });

  imageSrc.value = convertFileSrc("C:\\data\\1.png");
  iframeSrc.value = convertFileSrc("C:\\data\\1\\index.html");

  console.log(iframeSrc.value);
});
</script>

<style lang="scss">
.container {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  flex-direction: column;
  .header {
    flex: 0 0 50px;
    height: 50px;
    border-bottom: 1px solid #888;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .content {
    flex: 1;
    display: flex;
    height: 100%;
    .left {
      flex: 1;
      video {
        width: 100%;
      }
    }
    .center {
      flex: 1;
      border-left: 1px solid #888;
      border-right: 1px solid #888;
      img {
        width: 100%;
      }
    }
    .right {
      flex: 1;
    }
  }
}
</style>
