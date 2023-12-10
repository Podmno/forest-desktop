<script setup lang="ts">
import '@mdui/icons/warning';
import '@mdui/icons/info';
import { ref } from '@vue/reactivity';
import { onMounted } from 'vue';
import { snackbar } from 'mdui/functions/snackbar.js';
const btnRecordNotice = ref()


onMounted(() => {
    btnRecordNotice.value.addEventListener("click", () => {
        snackbar({
      message: "列表中仅显示在此软件中种下的树。",
    });
    })
})

</script>

<template>


    <div class="mdui-prose" style="margin-left: 15px;margin-top: 15px;margin-bottom: 15px;">

        <h2>记录</h2>
        
        
        <div style="display: flex;gap:50px">

        <div style="width: 120px;height: 60px;">
            <h4>总专注时长</h4>
            <h2 style="margin-top: -15px;">0h</h2>
        </div>
        <div style="width: 150px;height: 60px;">
            <h4>1 年内专注天数</h4>
            <h2 style="margin-top: -15px;">0</h2>
        </div>

        
        </div>
        <div style="margin-top: 15px;">
            <mdui-list>
  <mdui-list-subheader>最近 5 条专注记录</mdui-list-subheader>
  <mdui-list-item disabled>暂时没有记录</mdui-list-item>
  

</mdui-list>
        </div>
        <mdui-button-icon ref="btnRecordNotice" style="display: none;"><mdui-icon-info></mdui-icon-info></mdui-button-icon>
    </div>


</template>

<style scoped>
</style>
