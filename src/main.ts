import { createApp } from "vue";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import ConfirmationService from 'primevue/confirmationservice';
import Tooltip from "primevue/tooltip";
import { definePreset } from "@primeuix/themes";
import Aura from "@primeuix/themes/aura";
import 'primeicons/primeicons.css'
import WindowFrame from "./WindowFrame.vue";
import './styles/common.css';

//prevent right-click context menu on the entire document except for input and textarea elements
document.addEventListener('contextmenu', event => {
    if(event.target instanceof HTMLElement && event.target.tagName !== 'INPUT' && event.target.tagName !== 'TEXTAREA') {
        event.preventDefault();
    }
});

const AppPreset = definePreset(Aura, {
    semantic: {
        primary: {
            50: '{zinc.50}',
            100: '{zinc.100}',
            200: '{zinc.200}',
            300: '{zinc.300}',
            400: '{zinc.400}',
            500: '{zinc.500}',
            600: '{zinc.600}',
            700: '{zinc.700}',
            800: '{zinc.800}',
            900: '{zinc.900}',
            950: '{zinc.950}'
        },
        colorScheme: {
            light: {
                primary: {
                    color: '{zinc.800}',
                    inverseColor: '#ffffff',
                }
            }
        }
    }
});

createApp(WindowFrame)
    .use(createPinia())
    .use(ConfirmationService)
    .directive("tooltip", Tooltip)
    .use(PrimeVue, {
        theme: {
            preset: AppPreset,
            options: {
                darkModeSelector: false
            }
        }
    })
    .mount("#app");