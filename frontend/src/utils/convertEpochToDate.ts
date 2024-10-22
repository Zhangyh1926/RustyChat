import { SystemTime } from "./types";

export function convertEpochToDate(lastMessageTime: SystemTime): Date {
    // 将秒数转为毫秒
    const milliseconds = lastMessageTime.secs_since_epoch * 1000;
    // 将纳秒转换为毫秒并加到总的时间上
    const totalMilliseconds = milliseconds + lastMessageTime.nanos_since_epoch / 1_000_000;
    // 返回 Date 对象
    return new Date(totalMilliseconds);
}