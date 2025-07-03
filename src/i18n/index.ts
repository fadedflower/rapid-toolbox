import en from './en.json';
import zhCN from './zh-CN.json';

export type MessageSchema = typeof en;
export type Locales = "en" | "zh-CN";

export default {
    en,
    "zh-CN": zhCN
};