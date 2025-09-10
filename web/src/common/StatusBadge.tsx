import React from 'react';
import { Chip, ChipProps } from '@mui/material';

type StatusSeverity =
  | 'Critical'
  | 'Emergency'
  | 'Warning'
  | 'Info'
  | 'Success'
  | 'Online'
  | 'Offline'
  | 'Maintenance'
  | string;

interface StatusBadgeProps extends Omit<ChipProps, 'label' | 'color'> {
  status: string;
  severity?: StatusSeverity;
}

const StatusBadge: React.FC<StatusBadgeProps> = ({ status, severity, ...props }) => {
  const getChipColor = (): ChipProps['color'] => {
    switch (severity || status) {
      case 'Critical':
      case 'Emergency':
        return 'error';
      case 'Warning':
      case 'Maintenance':
        return 'warning';
      case 'Info':
        return 'info';
      case 'Success':
      case 'Online':
        return 'success';
      case 'Offline':
        return 'default';
      default:
        return 'default';
    }
  };

  return (
    <Chip
      label={status}
      color={getChipColor()}
      size="small"
      {...props}
    />
  );
};

export default StatusBadge;
