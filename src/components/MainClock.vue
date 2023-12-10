<script setup lang="ts">
import '@mdui/icons/warning';
import '@mdui/icons/play-circle';
import '@mdui/icons/tag';
import '@mdui/icons/minimize';
import '@mdui/icons/add';
import '@mdui/icons/arrow-back-ios';
import '@mdui/icons/arrow-forward-ios';

import {ref} from '@vue/reactivity'
import { onMounted } from 'vue';

import {getCurrentInstance} from 'vue';
const instance = getCurrentInstance();

const btnTimerAdd = ref()
const btnTimerMinus = ref()
const btnTimerStart = ref()

let valueFocusTimeArray = [10,15,20,25,30,35,40,45,50,55,60,65,70,75,80,85,90,95,100,105,110,115,120]
let valueFocusTimeCurrent = 3

let valueFocusTimeString = "25:00"

onMounted(() => {
    btnTimerAdd.value.addEventListener("click", () => {
        valueFocusTimeCurrent = valueFocusTimeCurrent + 1

        if(valueFocusTimeCurrent == valueFocusTimeArray.length - 1) {
            btnTimerAdd.value.disabled = true
        } else {
            btnTimerMinus.value.disabled = false
        }

        valueFocusTimeString = valueFocusTimeArray[valueFocusTimeCurrent] + ":00"

        instance!.proxy!.$forceUpdate();
    })

    btnTimerMinus.value.addEventListener("click", () => {

        valueFocusTimeCurrent = valueFocusTimeCurrent - 1
        if(valueFocusTimeCurrent == 0) {
            btnTimerMinus.value.disabled = true
        } else {
            btnTimerAdd.value.disabled = false
        }

        valueFocusTimeString = valueFocusTimeArray[valueFocusTimeCurrent] + ":00"

        instance!.proxy!.$forceUpdate();
    })

    btnTimerStart.value.addEventListener("click", () => {
        console.log("start timer!")
    })

})


</script>

<template>


    <div class="mdui-prose" style="margin-top: 80px;margin-right: 15px;margin-bottom: 15px;">

        <div style="width: 100%;height: 290px;flex-direction: column;justify-content: center;align-items: center;text-align: center;">
            
            <mdui-circular-progress style="width: 250px;height: 250px;
 margin: auto; --mdui-color-primary:212,232,209" value="1"></mdui-circular-progress>
 <h1 style="position: relative;width:100%;top:-165px; text-align: center; font-weight: 300; font-size:40pt;">
    {{ valueFocusTimeString }}
</h1>
            
            
        </div>

        <div style="display: flex;;margin-top: 0px; justify-content: center;width: 100%; gap: 10px;">
            <mdui-button-icon ref="btnTimerMinus"><mdui-icon-arrow-back-ios></mdui-icon-arrow-back-ios></mdui-button-icon>
            <mdui-button ref="btnTimerStart" style="width: 200px;">
            开始
                <mdui-icon-play-circle slot="icon"></mdui-icon-play-circle>
        </mdui-button>
        <mdui-button-icon ref="btnTimerAdd"><mdui-icon-arrow-forward-ios></mdui-icon-arrow-forward-ios></mdui-button-icon>
        </div>
        <div style="display: none;  margin-top: 30px;gap:10px;justify-content: center;">
            <mdui-chip selectable>雪松</mdui-chip>
            <mdui-chip selectable>猫咪</mdui-chip>
            <mdui-chip selectable>狗狗树</mdui-chip>
            
        </div>
        <div style="display: flex;;margin-top: 10px; justify-content: center;width: 100%;">
            <mdui-dropdown>
  <mdui-button slot="trigger" variant="text" style="display: none;">
    <mdui-icon-tag slot="icon"></mdui-icon-tag>
    标签</mdui-button>
  <mdui-menu>
    <mdui-menu-item disabled>没有可用标签</mdui-menu-item>
  </mdui-menu>
</mdui-dropdown>
        </div>

    </div>


</template>

<style scoped>
</style>
