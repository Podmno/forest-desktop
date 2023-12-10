<script setup lang="ts">
import '@mdui/icons/search';
import '@mdui/icons/menu';
import '@mdui/icons/timer';
import '@mdui/icons/calendar-month';
import '@mdui/icons/account-circle'
import '@mdui/icons/more-vert';
import '@mdui/icons/warning';
import '@mdui/icons/sync';
import 'mdui/mdui.css';
import 'mdui';

import { onMounted } from 'vue';
import { ref } from '@vue/reactivity';
import {getCurrentInstance} from 'vue';
const instance = getCurrentInstance();


import AccountManage from './components/AccountManage.vue'
import MainClock from './components/MainClock.vue'
import UserRecord from './components/UserRecord.vue'

const dialogOpenSource = ref()
const dialogInfo = ref()
const btnOpenSource = ref()
const btnCloseOpenSourceDialog = ref()
const btnInfo = ref()
const btnCloseInfoDialog = ref()

const navigationBar = ref()
const navAccountPage = ref()

let displayAccountPage = false
let displayClockPage = true
let displayRecordPage = false



onMounted(() => {
  dialogOpenSource.value.open = false
  btnOpenSource.value.addEventListener("click", () => dialogOpenSource.value.open = true)
  btnCloseOpenSourceDialog.value.addEventListener("click", () => dialogOpenSource.value.open = false)

  dialogInfo.value.open = false
  btnInfo.value.addEventListener("click", () => dialogInfo.value.open = true)
  btnCloseInfoDialog.value.addEventListener("click", () => dialogInfo.value.open = false)




  navigationBar.value.addEventListener("change", () => {

    let current_select = navigationBar.value.value
    if(current_select == 'item-1') {
      displayClockPage = true
      displayRecordPage = false
      displayAccountPage = false
    }
    if(current_select == 'item-2') {
      displayClockPage = false
      displayRecordPage = true
      displayAccountPage = false
    }
    if(current_select == 'item-3') {
      displayClockPage = false
      displayRecordPage = false
      displayAccountPage = true
    }


    
    instance!.proxy!.$forceUpdate();
  })
  
})




</script>

<template>
<div class="mdui-prose">
<mdui-dialog ref="dialogOpenSource" 
  headline="开放源代码许可">
  <mdui-list>
  <mdui-list-item headline="mdui / zdhxiong" href="https://github.com/zdhxiong/mdui" target="_blank"  description="MIT License"></mdui-list-item>
  <mdui-list-item headline="tauri / tauri-apps" href="https://github.com/tauri-apps/tauri" target="_blank" description="MIT License"></mdui-list-item>
</mdui-list>
  <mdui-button slot="action" variant="tonal" ref="btnCloseOpenSourceDialog">完成</mdui-button>
</mdui-dialog>

<mdui-dialog ref="dialogInfo" >
<div>⚠︎ 此软件与 SEEKRTECH CO., LTD. 开发的 Forest 专注森林无关，软件功能随时可能因为官方更新而无法使用。</div>

  <mdui-button slot="action" variant="tonal" ref="btnCloseInfoDialog">完成</mdui-button>
</mdui-dialog>

  <div>

    <mdui-top-app-bar style="position: relative;">


  <div style="flex-grow: 1"></div>


  <mdui-dropdown>
    <mdui-button-icon slot="trigger"><mdui-icon><mdui-icon-more-vert></mdui-icon-more-vert></mdui-icon></mdui-button-icon>
  <mdui-menu>
    <mdui-menu-item ref="btnInfo" style="display: none;">⚠︎ 使用前必读</mdui-menu-item>
    <mdui-menu-item ref="btnOpenSource">开放源代码许可</mdui-menu-item>
    <mdui-menu-item disabled>版本 1.0</mdui-menu-item>
  </mdui-menu>
</mdui-dropdown>

</mdui-top-app-bar>

<AccountManage v-if="displayAccountPage" :key="1" style="width: 90%;"></AccountManage>
<MainClock v-if="displayClockPage" :key="1" style="width: 90%;"></MainClock>
<UserRecord v-if="displayRecordPage" :key="1" style="width: 90%;"></UserRecord>

<mdui-navigation-rail alignment="center" contained value="item-1" ref="navigationBar" style=" z-index: 0">
  <mdui-navigation-rail-item value="item-1">
    时钟
    <mdui-icon slot="icon"><mdui-icon-timer></mdui-icon-timer></mdui-icon>
  </mdui-navigation-rail-item>
  <mdui-navigation-rail-item value="item-2">
    记录
    <mdui-icon slot="icon"><mdui-icon-calendar-month></mdui-icon-calendar-month></mdui-icon>
  </mdui-navigation-rail-item>
  <mdui-navigation-rail-item ref="navAccountPage" value="item-3">
    同步
    <mdui-icon slot="icon"><mdui-icon-sync></mdui-icon-sync></mdui-icon>
  </mdui-navigation-rail-item>
</mdui-navigation-rail>

  </div>
</div>
</template>

<style scoped>
*{ 
 -webkit-touch-callout:none; /*系统默认菜单被禁用*/ 
 -webkit-user-select:none; /*webkit浏览器*/ 
 -khtml-user-select:none; /*早期浏览器*/ 
 -moz-user-select:none;/*火狐*/ 
 -ms-user-select:none; /*IE10*/ 
 user-select:none; 
} 
input{ 
 -webkit-user-select:auto; /*webkit浏览器*/  
}
textarea{
 -webkit-user-select:auto; /*webkit浏览器*/
}

</style>
