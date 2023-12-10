<script setup lang="ts">
import '@mdui/icons/warning';
import '@mdui/icons/account-circle'
import '@mdui/icons/arrow-back';
import '@mdui/icons/close';

import { snackbar } from 'mdui/functions/snackbar.js';
import { ref } from '@vue/reactivity';
import { onMounted } from 'vue';
import 'mdui/mdui.css';
import 'mdui';

const btnLoginForestAccount = ref()
const dialogLogin = ref()
const btnLoginClose = ref()
const btnLoginPageLogin = ref()

const textEmail = ref()
const textPassword = ref()

function webPageCheckInput() {
    console.log("begin login process")

    let email = textEmail.value.value
    let passwd = textPassword.value.value

    if(email == '' || passwd == '') {
    snackbar({
      message: "请将电子邮箱与密码填写完整。",
    });
        return
    }

    
    btnLoginForestAccount.value.disabled = true
    btnLoginPageLogin.value.loading = true
    
    btnLoginClose.value.disabled = true

}

onMounted(() => {
    btnLoginForestAccount.value.addEventListener("click", () => dialogLogin.value.open = true)
    btnLoginClose.value.addEventListener("click", () => dialogLogin.value.open = false)
    btnLoginPageLogin.value.addEventListener("click", () => {
        webPageCheckInput()
    })
})

</script>

<template>
    <mdui-dialog fullscreen class="mdui-prose" ref="dialogLogin">
        <mdui-button-icon ref="btnLoginClose" style="margin-top: 10px;"><mdui-icon-close></mdui-icon-close></mdui-button-icon>
  <h1 style="margin-top: 70px;">登录 Forest</h1>
  <mdui-radio-group value="world">
  <mdui-radio value="world">全球服务器</mdui-radio>
  <mdui-radio style="display: none;" value="china">中国大陆服务器</mdui-radio>
</mdui-radio-group>
  <mdui-text-field ref="textEmail" style="margin-top: 10px;" toggle-password label="电子邮箱"></mdui-text-field>
  <mdui-text-field ref="textPassword" style="margin-top: 10px;" type="password" toggle-password label="密码"></mdui-text-field>
  <mdui-button full-width style="margin-top: 30px;" ref="btnLoginPageLogin">登录</mdui-button>

  <sub style="margin-left: 10px; margin-top: 10px;">⚠︎ 此服务非官方服务。</sub>
</mdui-dialog>

    <div class="mdui-prose" style="margin-left: 15px;margin-top: 15px; margin-bottom: 10px;">
        <h2>同步</h2>
        <mdui-card style="width: 90%; height: 100px;">
            <div style="display: flex;height: 50px;">
                <mdui-avatar style="margin-top: 10px; margin-left: 15px;">
                    <mdui-icon-account-circle></mdui-icon-account-circle>
                </mdui-avatar>
                <p style="margin-left: 15px; margin-top: 17px;font-weight: bold;">未登录</p>
            </div>
            
            <sub style="margin-left: 70px; margin-top: 10px;">本地账户</sub>
        </mdui-card>
        <br/>
        <div style="display: flex;margin-top: 20px;gap:10px">
            <mdui-button style="" ref="btnLoginForestAccount">登录 Forest 账户</mdui-button>
            <mdui-button style="display: none;">退出登录</mdui-button>
        <mdui-button variant="elevated" style="margin-left: 20px; display: none;">退出登录</mdui-button>

        
        </div>
        <div style="margin-top: 15px;">
            <mdui-list>
  <mdui-list-subheader>Forest 专用功能</mdui-list-subheader>
  <mdui-list-item disabled rounded description="雪松">使用树种</mdui-list-item>
  

</mdui-list>
        </div>
        <div class="mdui-prose" style="margin-top: 20px;line-height: 10px;">
            <sub style="display: none;">用户信息将会保存在本地，删除软件用户信息也将消失。<div><br/></div>
        无对应的「深度专注」功能，专注记录不会统计入排行榜。<div><br/></div>
        账号的更多功能，例如树种购买、修改密码、好友排行等请使用 Forest 手机版 App。<div><br/></div>
        ⚠︎ 此服务非官方服务。
        </sub>
    </div>
    </div>


</template>

<style scoped>
</style>
