import { v4 as uuidv4 } from 'uuid';

export const generateId = () => {
  return uuidv4();
};

export const debounce = (func, wait) => {
  let timeout;
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout);
      func(...args);
    };
    clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
};

export const throttle = (func, limit) => {
  let inThrottle;
  return function executedFunction(...args) {
    if (!inThrottle) {
      func(...args);
      inThrottle = true;
      setTimeout(() => inThrottle = false, limit);
    }
  };
};

export const groupBy = (array, key) => {
  return array.reduce((result, currentValue) => {
    (result[currentValue[key]] = result[currentValue[key]] || []).push(currentValue);
    return result;
  }, {});
};

export const sortBy = (array, key, order = 'asc') => {
  return [...array].sort((a, b) => {
    if (order === 'asc') {
      return a[key] > b[key] ? 1 : -1;
    } else {
      return a[key] < b[key] ? 1 : -1;
    }
  });
};

export const filterBy = (array, filters) => {
  return array.filter(item => {
    return Object.keys(filters).every(key => {
      if (!filters[key]) return true;
      return item[key] && item[key].toString().toLowerCase().includes(filters[key].toString().toLowerCase());
    });
  });
};

export const paginate = (array, page, pageSize) => {
  const startIndex = (page - 1) * pageSize;
  const endIndex = startIndex + pageSize;
  return array.slice(startIndex, endIndex);
};

export const calculateStats = (array, key) => {
  if (array.length === 0) return { min: 0, max: 0, avg: 0, sum: 0, count: 0 };
  
  const values = array.map(item => item[key]).filter(v => v !== null && v !== undefined);
  if (values.length === 0) return { min: 0, max: 0, avg: 0, sum: 0, count: 0 };
  
  const min = Math.min(...values);
  const max = Math.max(...values);
  const sum = values.reduce((acc, val) => acc + val, 0);
  const avg = sum / values.length;
  const count = values.length;
  
  return { min, max, avg, sum, count };
};

export const calculateTrend = (array, key) => {
  if (array.length < 2) return 'stable';
  
  const first = array[0][key];
  const last = array[array.length - 1][key];
  
  if (last > first * 1.1) return 'increasing';
  if (last < first * 0.9) return 'decreasing';
  return 'stable';
};

export const formatCurrency = (amount, currency = 'USD') => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: currency,
    minimumFractionDigits: 2,
  }).format(amount);
};

export const formatDistance = (distance, unit = 'km') => {
  if (unit === 'km') {
    return `${distance.toFixed(2)} km`;
  } else {
    return `${(distance * 0.621371).toFixed(2)} mi`;
  }
};

export const formatWeight = (weight, unit = 'kg') => {
  if (unit === 'kg') {
    return `${weight.toFixed(2)} kg`;
  } else {
    return `${(weight * 2.20462).toFixed(2)} lbs`;
  }
};

export const formatVolume = (volume, unit = 'L') => {
  if (unit === 'L') {
    return `${volume.toFixed(2)} L`;
  } else {
    return `${(volume * 0.264172).toFixed(2)} gal`;
  }
};

export const getSeverityColor = (severity) => {
  switch (severity) {
    case 'Critical':
    case 'Emergency':
      return 'error';
    case 'Warning':
      return 'warning';
    case 'Info':
      return 'info';
    default:
      return 'default';
  }
};

export const getStatusColor = (status) => {
  switch (status) {
    case 'Online':
    case 'Success':
      return 'success';
    case 'Offline':
    case 'Failed':
      return 'error';
    case 'Maintenance':
    case 'Warning':
      return 'warning';
    case 'Pending':
      return 'info';
    default:
      return 'default';
  }
};

export const getHealthScoreColor = (score) => {
  if (score >= 80) return 'success';
  if (score >= 60) return 'warning';
  return 'error';
};

export const getResourceUsageColor = (usage) => {
  if (usage > 90) return 'error';
  if (usage > 80) return 'warning';
  if (usage > 60) return 'primary';
  return 'success';
};

export const getConfidenceColor = (confidence) => {
  if (confidence > 0.9) return 'success';
  if (confidence > 0.8) return 'warning';
  return 'primary';
};

export const getProgressColor = (progress) => {
  if (progress > 90) return 'success';
  if (progress > 70) return 'warning';
  if (progress > 50) return 'primary';
  return 'info';
};

export const truncateText = (text, maxLength = 50) => {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
};

export const capitalize = (str) => {
  if (!str) return '';
  return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
};

export const camelToTitle = (str) => {
  if (!str) return '';
  return str
    .replace(/([A-Z])/g, ' $1')
    .replace(/^./, (str) => str.toUpperCase());
};

export const downloadFile = (url, filename) => {
  const link = document.createElement('a');
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
};

export const copyToClipboard = (text) => {
  navigator.clipboard.writeText(text);
};

export const isValidEmail = (email) => {
  const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return re.test(String(email).toLowerCase());
};

export const isValidPhone = (phone) => {
  const re = /^[\+]?[1-9][\d]{0,15}$/;
  return re.test(String(phone));
};

export const isValidURL = (url) => {
  try {
    new URL(url);
    return true;
  } catch (err) {
    return false;
  }
};

export const sleep = (ms) => {
  return new Promise(resolve => setTimeout(resolve, ms));
};