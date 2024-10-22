// src/views/timeFormatter.ts

export const formatLastMessageTime = (timestamp: Date): string => {
    const now = new Date();
    const time = new Date(timestamp);
    const diff = now.getTime() - time.getTime() + 8 * 60 * 60 * 1000; // because the server is in UTC+8 timezone
  
    const oneMinute = 1000 * 60;
    const oneHour = oneMinute * 60;
    const oneDay = oneHour * 24;
    const oneWeek = oneDay * 7;
    const oneYear = oneDay * 365;
  
    if (diff < oneMinute) {
      return '刚刚';
    } else if (diff < oneHour) {
      return `${Math.floor(diff / oneMinute)} 分钟前`;
    } else if (diff < oneDay) {
      return `${time.getHours()}:${time.getMinutes().toString().padStart(2, '0')}`;
    } else if (diff < oneWeek) {
      return ['星期天', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六'][time.getDay()];
    } else if (diff < oneYear) {
      return `${time.getMonth() + 1}月${time.getDate()}日`;
    } else {
      return `${time.getFullYear()}年${time.getMonth() + 1}月${time.getDate()}日`;
    }
  };
