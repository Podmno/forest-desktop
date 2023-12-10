import { createApp } from 'vue'
import 'mdui/mdui.css';
import 'mdui';
import './style.css'
import App from './App.vue'

import { setColorScheme } from 'mdui/functions/setColorScheme.js';

setColorScheme('#006e1c');

createApp(App).mount('#app')
