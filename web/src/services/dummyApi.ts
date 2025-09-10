import { v4 as uuidv4 } from 'uuid';

class DummyApiService {
  constructor() {
    this.initializeLocalStorage();
  }

  // Initialize local storage with seed data if not exists
  initializeLocalStorage() {
    // Check if data already exists
    if (!localStorage.getItem('fleetDataInitialized')) {
      this.seedData();
      localStorage.setItem('fleetDataInitialized', 'true');
    }
  }

  // Generate realistic dummy data
  seedData() {
    console.log('🌱 Seeding dummy data...');
    
    // Create trucks
    const trucks = this.generateTrucks(10);
    localStorage.setItem('trucks', JSON.stringify(trucks));
    
    // Create telemetry data
    const telemetry = this.generateTelemetryData(trucks);
    localStorage.setItem('telemetry', JSON.stringify(telemetry));
    
    // Create alerts
    const alerts = this.generateAlerts(trucks);
    localStorage.setItem('alerts', JSON.stringify(alerts));
    
    // Create ML events
    const mlEvents = this.generateMlEvents(trucks);
    localStorage.setItem('mlEvents', JSON.stringify(mlEvents));
    
    // Create health status
    const healthStatus = this.generateHealthStatus(trucks);
    localStorage.setItem('healthStatus', JSON.stringify(healthStatus));
    
    // Create OTA updates
    const otaUpdates = this.generateOtaUpdates(trucks);
    localStorage.setItem('otaUpdates', JSON.stringify(otaUpdates));
    
    // Create remote commands
    const remoteCommands = this.generateRemoteCommands(trucks);
    localStorage.setItem('remoteCommands', JSON.stringify(remoteCommands));
    
    console.log('✅ Dummy data seeded successfully');
  }

  // Generate dummy trucks
  generateTrucks(count:number) {
    const makes = ['Volvo', 'Scania', 'Mercedes', 'MAN', 'DAF'];
    const models = ['FH16', 'R-series', 'Actros', 'TGX', 'XF'];
    const years = ['2020', '2021', '2022', '2023'];
    const statuses = ['Online', 'Offline', 'Maintenance'];
    
    const trucks = [];
    const baseLat = 37.7749;
    const baseLng = -122.4194;
    
    for (let i = 1; i <= count; i++) {
      const make = makes[Math.floor(Math.random() * makes.length)];
      const model = models[Math.floor(Math.random() * models.length)];
      const year = years[Math.floor(Math.random() * years.length)];
      const status = statuses[Math.floor(Math.random() * statuses.length)];
      
      // Generate realistic location with slight variation
      const lat = baseLat + (Math.random() - 0.5) * 0.1;
      const lng = baseLng + (Math.random() - 0.5) * 0.1;
      
      trucks.push({
        id: uuidv4(),
        truck_id: `TRK-${String(i).padStart(4, '0')}`,
        model,
        make,
        year,
        license_plate: `TRK${String(i).padStart(3, '0')}A`,
        vin: `VIN${uuidv4().substring(0, 17)}`,
        fleet_id: i % 3 === 0 ? uuidv4() : null,
        driver_id: i % 2 === 0 ? uuidv4() : null,
        status,
        last_seen: new Date(Date.now() - Math.random() * 3600000).toISOString(), // Last hour
        location: [lng, lat],
        created_at: new Date(Date.now() - Math.random() * 30 * 24 * 3600000).toISOString(), // Last 30 days
        updated_at: new Date().toISOString(),
      });
    }
    
    return trucks;
  }

  // Generate telemetry data for trucks
  generateTelemetryData(trucks) {
    const telemetry = [];
    const now = Date.now();
    
    trucks.forEach(truck => {
      // Generate 50 telemetry points per truck for the last 24 hours
      for (let i = 0; i < 50; i++) {
        const timestamp = new Date(now - (49 - i) * 30 * 60000).toISOString(); // Every 30 minutes
        
        // Generate realistic sensor data
        const speed_kmh = Math.random() * 100;
        const heading = Math.random() * 360;
        
        telemetry.push({
          id: uuidv4(),
          truck_id: truck.id,
          timestamp,
          location: [
            truck.location[0] + (Math.random() - 0.5) * 0.001,
            truck.location[1] + (Math.random() - 0.5) * 0.001
          ],
          speed_kmh,
          heading,
          sensors: {
            gps: {
              latitude: truck.location[1] + (Math.random() - 0.5) * 0.001,
              longitude: truck.location[0] + (Math.random() - 0.5) * 0.001,
              altitude: 100 + Math.random() * 50,
              speed_kmh,
              heading,
              satellites: 8 + Math.floor(Math.random() * 4),
              fix_quality: 1,
            },
            obd: {
              rpm: 1000 + Math.floor(Math.random() * 3000),
              speed_kmh: Math.floor(speed_kmh),
              coolant_temp: 80 + Math.floor(Math.random() * 20) - 10,
              fuel_level: 50 + Math.floor(Math.random() * 50),
              engine_load: 30 + Math.floor(Math.random() * 70),
              throttle_pos: 20 + Math.floor(Math.random() * 80),
            },
            imu: {
              accel_x: (Math.random() - 0.5) * 2,
              accel_y: (Math.random() - 0.5) * 2,
              accel_z: 0.98 + (Math.random() - 0.5) * 0.1,
              gyro_x: (Math.random() - 0.5) * 10,
              gyro_y: (Math.random() - 0.5) * 10,
              gyro_z: (Math.random() - 0.5) * 10,
            },
            tpms: {
              front_left: {
                pressure_psi: 32 + (Math.random() - 0.5) * 4,
                temperature_c: 25 + Math.random() * 20,
                battery_percent: 80 + Math.floor(Math.random() * 20),
                alert: Math.random() > 0.95,
              },
              front_right: {
                pressure_psi: 32 + (Math.random() - 0.5) * 4,
                temperature_c: 25 + Math.random() * 20,
                battery_percent: 80 + Math.floor(Math.random() * 20),
                alert: Math.random() > 0.95,
              },
              rear_left: {
                pressure_psi: 32 + (Math.random() - 0.5) * 4,
                temperature_c: 25 + Math.random() * 20,
                battery_percent: 80 + Math.floor(Math.random() * 20),
                alert: Math.random() > 0.95,
              },
              rear_right: {
                pressure_psi: 32 + (Math.random() - 0.5) * 4,
                temperature_c: 25 + Math.random() * 20,
                battery_percent: 80 + Math.floor(Math.random() * 20),
                alert: Math.random() > 0.95,
              },
            },
          },
          cameras: {
            front_camera: i % 10 === 0 ? {
              frame_id: uuidv4(),
              timestamp,
              url: `https://picsum.photos/1280/720?random=${Math.floor(Math.random() * 1000)}`,
              thumbnail_url: `https://picsum.photos/320/180?random=${Math.floor(Math.random() * 1000)}`,
              width: 1280,
              height: 720,
              format: "jpeg",
              size_bytes: 1024 * 1024,
              is_keyframe: true,
              meta: {
                exposure_us: 10000,
                gain_db: 0.0,
                temperature_c: 25.0,
                gps_lat: truck.location[1],
                gps_lon: truck.location[0],
                speed_kmh,
              },
            } : null,
            driver_camera: i % 5 === 0 ? {
              frame_id: uuidv4(),
              timestamp,
              url: `https://picsum.photos/640/480?random=${Math.floor(Math.random() * 1000)}`,
              thumbnail_url: `https://picsum.photos/160/120?random=${Math.floor(Math.random() * 1000)}`,
              width: 640,
              height: 480,
              format: "jpeg",
              size_bytes: 512 * 1024,
              is_keyframe: true,
              meta: {
                exposure_us: 8000,
                gain_db: 0.0,
                temperature_c: 25.0,
                gps_lat: truck.location[1],
                gps_lon: truck.location[0],
                speed_kmh,
              },
            } : null,
            cargo_camera: i % 7 === 0 ? {
              frame_id: uuidv4(),
              timestamp,
              url: `https://picsum.photos/800/600?random=${Math.floor(Math.random() * 1000)}`,
              thumbnail_url: `https://picsum.photos/200/150?random=${Math.floor(Math.random() * 1000)}`,
              width: 800,
              height: 600,
              format: "jpeg",
              size_bytes: 768 * 1024,
              is_keyframe: true,
              meta: {
                exposure_us: 12000,
                gain_db: 0.0,
                temperature_c: 25.0,
                gps_lat: truck.location[1],
                gps_lon: truck.location[0],
                speed_kmh,
              },
            } : null,
          },
          scenario: ['normal_driving', 'emergency_braking', 'rapid_acceleration', 'sharp_turn'][Math.floor(Math.random() * 4)],
          created_at: timestamp,
        });
      }
    });
    
    return telemetry;
  }

  // Generate alerts for trucks
  generateAlerts(trucks) {
    const alertTypes = [
      'DrowsyDriver', 'LaneDeparture', 'CargoTamper', 
      'HarshBraking', 'RapidAcceleration', 'OverSpeeding',
      'HighTemperature', 'LowDiskSpace', 'HighCpuUsage'
    ];
    const severities = ['Info', 'Warning', 'Critical', 'Emergency'];
    const statuses = ['Triggered', 'Acknowledged', 'Resolved'];
    
    const alerts = [];
    const now = Date.now();
    
    trucks.forEach(truck => {
      // Generate 5-15 alerts per truck
      const alertCount = 5 + Math.floor(Math.random() * 11);
      
      for (let i = 0; i < alertCount; i++) {
        const alertType = alertTypes[Math.floor(Math.random() * alertTypes.length)];
        const severity = severities[Math.floor(Math.random() * severities.length)];
        const status = statuses[Math.floor(Math.random() * statuses.length)];
        
        // Generate realistic triggered time (last 7 days)
        const triggeredAt = new Date(now - Math.random() * 7 * 24 * 3600000).toISOString();
        
        let acknowledgedAt = null;
        let resolvedAt = null;
        
        if (status === 'Acknowledged' || status === 'Resolved') {
          acknowledgedAt = new Date(new Date(triggeredAt).getTime() + Math.random() * 3600000).toISOString();
        }
        
        if (status === 'Resolved') {
          resolvedAt = new Date(new Date(acknowledgedAt || triggeredAt).getTime() + Math.random() * 3600000).toISOString();
        }
        
        alerts.push({
          id: uuidv4(),
          alert_id: `ALERT-${uuidv4().substring(0, 8)}`,
          truck_id: truck.id,
          alert_type: alertType,
          severity,
          message: this.generateAlertMessage(alertType, severity),
          triggered_at: triggeredAt,
          acknowledged_at: acknowledgedAt,
          resolved_at: resolvedAt,
          source: 'dummy_generator',
          context: this.generateAlertContext(alertType, truck),
          actions: this.generateAlertActions(alertType, severity),
          status,
          created_at: triggeredAt,
          updated_at: resolvedAt || acknowledgedAt || triggeredAt,
        });
      }
    });
    
    return alerts;
  }

  // Generate alert message based on type and severity
  generateAlertMessage(alertType, severity) {
    const messages = {
      DrowsyDriver: {
        Info: 'Driver showing signs of drowsiness',
        Warning: 'Driver drowsiness detected - monitor closely',
        Critical: 'Driver drowsiness detected - immediate attention required',
        Emergency: 'Driver asleep at wheel - emergency stop required'
      },
      LaneDeparture: {
        Info: 'Minor lane departure detected',
        Warning: 'Lane departure detected - correct steering',
        Critical: 'Severe lane departure detected - immediate correction required',
        Emergency: 'Vehicle leaving roadway - emergency intervention required'
      },
      CargoTamper: {
        Info: 'Possible cargo movement detected',
        Warning: 'Cargo tampering detected - inspect cargo area',
        Critical: 'Cargo tampering confirmed - secure cargo immediately',
        Emergency: 'Cargo theft in progress - notify authorities immediately'
      },
      HarshBraking: {
        Info: 'Moderate braking detected',
        Warning: 'Harsh braking detected - review driving behavior',
        Critical: 'Emergency braking detected - check for accidents',
        Emergency: 'Collision detected - emergency response required'
      },
      RapidAcceleration: {
        Info: 'Aggressive acceleration detected',
        Warning: 'Rapid acceleration detected - review driving behavior',
        Critical: 'Dangerous acceleration detected - immediate intervention required',
        Emergency: 'Loss of control detected - emergency stop required'
      },
      OverSpeeding: {
        Info: 'Speed limit slightly exceeded',
        Warning: 'Speed limit significantly exceeded - slow down',
        Critical: 'Dangerous speeding detected - immediate intervention required',
        Emergency: 'Extreme speeding detected - emergency stop required'
      },
      HighTemperature: {
        Info: 'System temperature slightly elevated',
        Warning: 'System temperature high - monitor closely',
        Critical: 'System temperature critical - reduce load immediately',
        Emergency: 'System overheating - emergency shutdown required'
      },
      LowDiskSpace: {
        Info: 'Disk space running low',
        Warning: 'Disk space critically low - clean up space',
        Critical: 'Disk space almost full - immediate cleanup required',
        Emergency: 'Disk full - system may become unstable'
      },
      HighCpuUsage: {
        Info: 'CPU usage elevated',
        Warning: 'CPU usage high - monitor system performance',
        Critical: 'CPU usage critical - reduce load immediately',
        Emergency: 'System unresponsive - emergency restart required'
      }
    };
    
    return messages[alertType]?.[severity] || `${alertType} alert (${severity})`;
  }

  // Generate alert context
  generateAlertContext(alertType, truck) {
    const context = {
      truck_id: truck.id,
      truck_license_plate: truck.license_plate,
      truck_model: truck.model,
      truck_make: truck.make,
      location: truck.location,
    };
    
    switch (alertType) {
      case 'DrowsyDriver':
        return {
          ...context,
          eye_closure_ratio: 0.3 + Math.random() * 0.7,
          head_pose: {
            yaw: (Math.random() - 0.5) * 30,
            pitch: (Math.random() - 0.5) * 30,
            roll: (Math.random() - 0.5) * 30,
          },
          time_of_day: ['day', 'night', 'dusk'][Math.floor(Math.random() * 3)],
        };
      case 'LaneDeparture':
        return {
          ...context,
          deviation_pixels: 20 + Math.floor(Math.random() * 100),
          lane_confidence: 0.7 + Math.random() * 0.3,
          speed_kmh: 60 + Math.random() * 60,
        };
      case 'CargoTamper':
        return {
          ...context,
          motion_score: 0.5 + Math.random() * 0.5,
          object_count_change: Math.floor(Math.random() * 5) - 2,
        };
      case 'HarshBraking':
        return {
          ...context,
          g_force: 0.5 + Math.random() * 0.5,
          speed_kmh: 50 + Math.random() * 70,
        };
      case 'RapidAcceleration':
        return {
          ...context,
          g_force: 0.4 + Math.random() * 0.6,
          speed_kmh: 30 + Math.random() * 80,
        };
      case 'OverSpeeding':
        return {
          ...context,
          speed_kmh: 90 + Math.random() * 50,
          speed_limit: 80,
        };
      case 'HighTemperature':
        return {
          ...context,
          temperature_c: 70 + Math.random() * 30,
          cpu_percent: 80 + Math.random() * 20,
        };
      case 'LowDiskSpace':
        return {
          ...context,
          disk_percent: 85 + Math.random() * 15,
          disk_used_gb: 100 + Math.random() * 50,
          disk_total_gb: 150,
        };
      case 'HighCpuUsage':
        return {
          ...context,
          cpu_percent: 85 + Math.random() * 15,
          load_average: {
            '1m': 2 + Math.random() * 3,
            '5m': 1.5 + Math.random() * 2.5,
            '15m': 1 + Math.random() * 2,
          },
        };
      default:
        return context;
    }
  }

  // Generate alert actions
  generateAlertActions(alertType, severity) {
    const actions = [];
    
    if (severity === 'Critical' || severity === 'Emergency') {
      actions.push({
        action_id: `ACTION-${uuidv4().substring(0, 8)}`,
        action_type: 'TriggerBuzzer',
        target: 'buzzer_1',
        parameters: {
          duration_ms: 1000,
          pattern: 'pulse',
          pulse_count: 5,
        },
        executed_at: null,
        success: false,
        error: null,
      });
      
      actions.push({
        action_id: `ACTION-${uuidv4().substring(0, 8)}`,
        action_type: 'FlashLed',
        target: 'led_red',
        parameters: {
          duration_ms: 5000,
          pattern: 'blink',
          blink_count: 10,
        },
        executed_at: null,
        success: false,
        error: null,
      });
    }
    
    if (alertType === 'DrowsyDriver' && severity === 'Emergency') {
      actions.push({
        action_id: `ACTION-${uuidv4().substring(0, 8)}`,
        action_type: 'ShowOnDisplay',
        target: 'display_1',
        parameters: {
          message: 'DROWSY DRIVER - PULL OVER IMMEDIATELY',
          duration_ms: 10000,
        },
        executed_at: null,
        success: false,
        error: null,
      });
    }
    
    if (alertType === 'LaneDeparture' && severity === 'Critical') {
      actions.push({
        action_id: `ACTION-${uuidv4().substring(0, 8)}`,
        action_type: 'ShowOnDisplay',
        target: 'display_1',
        parameters: {
          message: 'LANE DEPARTURE - CORRECT STEERING',
          duration_ms: 5000,
        },
        executed_at: null,
        success: false,
        error: null,
      });
    }
    
    if (alertType === 'CargoTamper' && severity === 'Critical') {
      actions.push({
        action_id: `ACTION-${uuidv4().substring(0, 8)}`,
        action_type: 'ActivateRelay',
        target: 'relay_1',
        parameters: {
          activate: true,
          duration_ms: 0,
        },
        executed_at: null,
        success: false,
        error: null,
      });
    }
    
    return actions;
  }

  // Generate ML events for trucks
  generateMlEvents(trucks) {
    const modelNames = ['drowsiness', 'lane_departure', 'cargo_tamper', 'license_plate', 'weather'];
    const resultTypes = ['Drowsiness', 'LaneDeparture', 'CargoTamper', 'LicensePlate', 'Weather'];
    
    const mlEvents = [];
    const now = Date.now();
    
    trucks.forEach(truck => {
      // Generate 10-30 ML events per truck
      const eventCount = 10 + Math.floor(Math.random() * 21);
      
      for (let i = 0; i < eventCount; i++) {
        const modelName = modelNames[Math.floor(Math.random() * modelNames.length)];
        const resultType = resultTypes[modelNames.indexOf(modelName)];
        const confidence = 0.6 + Math.random() * 0.4; // 0.6-1.0
        const isAlert = confidence > 0.8;
        
        // Generate realistic timestamp (last 7 days)
        const timestamp = new Date(now - Math.random() * 7 * 24 * 3600000).toISOString();
        
        mlEvents.push({
          id: uuidv4(),
          event_id: `ML-${uuidv4().substring(0, 8)}`,
          truck_id: truck.id,
          model_name: modelName,
          model_version: '1.0.0',
          timestamp,
          result: this.generateMlResult(resultType),
          confidence,
          calibrated_confidence: confidence * (0.9 + Math.random() * 0.2),
          latency_ms: 30 + Math.random() * 70,
          hardware_used: ['Cpu', 'Cuda'][Math.floor(Math.random() * 2)],
          meta: {
            device_id: truck.truck_id,
            truck_id: truck.id,
            route_id: `ROUTE-${Math.floor(Math.random() * 100)}`,
            driver_id: truck.driver_id || uuidv4(),
            camera_id: ['driver_camera', 'front_camera', 'cargo_camera'][Math.floor(Math.random() * 3)],
            frame_timestamp: timestamp,
            sensor_context: {
              speed_kmh: 40 + Math.random() * 80,
              acceleration: (Math.random() - 0.5) * 2,
              steering_angle: (Math.random() - 0.5) * 90,
              gps_lat: truck.location[1],
              gps_lon: truck.location[0],
              time_of_day: ['day', 'night', 'dusk'][Math.floor(Math.random() * 3)],
            },
            cpu_usage_percent: 30 + Math.random() * 50,
            gpu_usage_percent: 20 + Math.random() * 60,
            memory_used_bytes: 512 * 1024 * 1024 + Math.random() * 512 * 1024 * 1024,
            temperature_c: 40 + Math.random() * 30,
            model_checksum: 'sha256:deadbeef...',
            retry_count: 0,
            fallback_reason: null,
          },
          created_at: timestamp,
        });
      }
    });
    
    return mlEvents;
  }

  // Generate ML result based on type
  generateMlResult(resultType) {
    switch (resultType) {
      case 'DrowsyDriver':
      case 'Drowsiness':
        return {
          is_drowsy: Math.random() > 0.5,
          eye_closure_ratio: 0.2 + Math.random() * 0.8,
        };
      case 'LaneDeparture':
        return {
          is_departing: Math.random() > 0.5,
          deviation_pixels: 10 + Math.floor(Math.random() * 100),
        };
      case 'CargoTamper':
        return {
          is_tampered: Math.random() > 0.5,
          motion_score: 0.3 + Math.random() * 0.7,
        };
      case 'LicensePlate':
        return {
          plate_text: `TRK${Math.floor(Math.random() * 10000)}`,
          bounding_box: [
            0.1 + Math.random() * 0.3,
            0.1 + Math.random() * 0.3,
            0.2 + Math.random() * 0.2,
            0.1 + Math.random() * 0.1,
          ],
        };
      case 'Weather':
        return {
          weather_type: ['Clear', 'Rain', 'Fog', 'Snow', 'Night'][Math.floor(Math.random() * 5)],
          visibility_m: 100 + Math.random() * 900,
        };
      default:
        return {};
    }
  }

  // Generate health status for trucks
  generateHealthStatus(trucks) {
    const healthStatusTypes = ['Ok', 'Warning', 'Critical', 'Degraded'];
    
    const healthStatus = [];
    const now = Date.now();
    
    trucks.forEach(truck => {
      // Generate 5-15 health status records per truck
      const statusCount = 5 + Math.floor(Math.random() * 11);
      
      for (let i = 0; i < statusCount; i++) {
        const timestamp = new Date(now - Math.random() * 7 * 24 * 3600000).toISOString();
        const cpuPercent = 30 + Math.random() * 60;
        const memoryPercent = 40 + Math.random() * 50;
        const diskPercent = 50 + Math.random() * 40;
        const temperatureC = 40 + Math.random() * 40;
        
        // Determine status based on metrics
        let status = 'Ok';
        if (cpuPercent > 85 || memoryPercent > 85 || diskPercent > 90 || temperatureC > 75) {
          status = 'Critical';
        } else if (cpuPercent > 75 || memoryPercent > 75 || diskPercent > 80 || temperatureC > 65) {
          status = 'Warning';
        } else if (cpuPercent > 65 || memoryPercent > 65 || diskPercent > 70) {
          status = 'Degraded';
        }
        
        healthStatus.push({
          id: uuidv4(),
          truck_id: truck.id,
          timestamp,
          status,
          resources: {
            cpu_percent: cpuPercent,
            cpu_cores: 4,
            memory_percent: memoryPercent,
            memory_used_mb: 2048 + Math.floor(Math.random() * 2048),
            memory_total_mb: 4096,
            memory_available_mb: 1024 + Math.floor(Math.random() * 3072),
            swap_percent: 10 + Math.random() * 40,
            disk_percent: diskPercent,
            disk_used_gb: 50 + Math.floor(Math.random() * 100),
            disk_total_gb: 200,
            disk_available_gb: 50 + Math.floor(Math.random() * 150),
            temperature_c: temperatureC,
            thermal_throttling: temperatureC > 80,
            uptime_sec: 3600 + Math.floor(Math.random() * 86400),
            load_average: [
              1 + Math.random() * 3,
              0.8 + Math.random() * 2.5,
              0.6 + Math.random() * 2,
            ],
          },
          tasks: this.generateTasks(),
          alerts: this.generateHealthAlerts(status),
          actions_taken: this.generateHealthActions(status),
          meta: {
            device_id: truck.truck_id,
            version: '1.0.0',
            hostname: `truck-${truck.truck_id.toLowerCase()}`,
            ip_address: `192.168.1.${10 + Math.floor(Math.random() * 245)}`,
            mac_address: `00:11:22:33:44:${Math.floor(Math.random() * 256).toString(16).padStart(2, '0')}`,
            location: truck.location,
            hardware_model: 'Raspberry Pi 4',
          },
          created_at: timestamp,
        });
      }
    });
    
    return healthStatus;
  }

  // Generate tasks for health status
  generateTasks() {
    const taskNames = ['sensor_engine', 'camera_engine', 'ml_engine', 'health_engine', 'ota_engine'];
    const statuses = ['Running', 'Degraded', 'Failed', 'Restarting'];
    
    const tasks = [];
    
    for (let i = 0; i < 5; i++) {
      const status = statuses[Math.floor(Math.random() * statuses.length)];
      const isAlive = status === 'Running';
      
      tasks.push({
        name: taskNames[i],
        is_alive: isAlive,
        last_seen_ms: Math.floor(Math.random() * 60000),
        cpu_usage_percent: 10 + Math.random() * 40,
        memory_usage_mb: 100 + Math.floor(Math.random() * 400),
        restarts: Math.floor(Math.random() * 5),
        last_restart: Math.random() > 0.8 ? new Date(Date.now() - Math.random() * 3600000).toISOString() : null,
      });
    }
    
    return tasks;
  }

  // Generate health alerts based on status
  generateHealthAlerts(status) {
    const alerts = [];
    
    if (status === 'Warning' || status === 'Critical') {
      if (Math.random() > 0.5) {
        alerts.push({
          alert_id: `HEALTH-${uuidv4().substring(0, 8)}`,
          alert_type: 'high_cpu_usage',
          severity: status === 'Critical' ? 'Critical' : 'Warning',
          message: status === 'Critical' ? 'CPU usage critical' : 'CPU usage high',
          triggered_at: new Date().toISOString(),
          source: 'health_monitor',
          recommended_action: 'Reduce load or restart service',
        });
      }
      
      if (Math.random() > 0.5) {
        alerts.push({
          alert_id: `HEALTH-${uuidv4().substring(0, 8)}`,
          alert_type: 'high_memory_usage',
          severity: status === 'Critical' ? 'Critical' : 'Warning',
          message: status === 'Critical' ? 'Memory usage critical' : 'Memory usage high',
          triggered_at: new Date().toISOString(),
          source: 'health_monitor',
          recommended_action: 'Clear cache or restart service',
        });
      }
      
      if (Math.random() > 0.5) {
        alerts.push({
          alert_id: `HEALTH-${uuidv4().substring(0, 8)}`,
          alert_type: 'high_temperature',
          severity: status === 'Critical' ? 'Critical' : 'Warning',
          message: status === 'Critical' ? 'System temperature critical' : 'System temperature high',
          triggered_at: new Date().toISOString(),
          source: 'health_monitor',
          recommended_action: 'Reduce load or check cooling',
        });
      }
    }
    
    return alerts;
  }

  // Generate health actions based on status
  generateHealthActions(status) {
    const actions = [];
    
    if (status === 'Warning' || status === 'Critical') {
      if (Math.random() > 0.5) {
        actions.push({
          action_id: `ACTION-${uuidv4().substring(0, 8)}`,
          action_type: 'ThrottleCameraFps',
          target_module: 'camera',
          parameters: {
            reduction_percent: 50,
          },
          executed_at: new Date().toISOString(),
          success: true,
          message: 'Reduced camera FPS to reduce load',
        });
      }
      
      if (Math.random() > 0.5) {
        actions.push({
          action_id: `ACTION-${uuidv4().substring(0, 8)}`,
          action_type: 'DisableMlModel',
          target_module: 'ml_edge',
          parameters: {
            model: 'license_plate',
          },
          executed_at: new Date().toISOString(),
          success: true,
          message: 'Disabled license plate model to reduce load',
        });
      }
      
      if (status === 'Critical' && Math.random() > 0.5) {
        actions.push({
          action_id: `ACTION-${uuidv4().substring(0, 8)}`,
          action_type: 'RebootSystem',
          target_module: 'system',
          parameters: {
            reason: 'critical_health',
          },
          executed_at: new Date().toISOString(),
          success: false,
          message: 'Scheduled system reboot due to critical health',
        });
      }
    }
    
    return actions;
  }

  // Generate OTA updates for trucks
  generateOtaUpdates(trucks) {
    const targets = ['Agent', 'Model', 'Config', 'Firmware'];
    const priorities = ['Critical', 'High', 'Medium', 'Low'];
    const statuses = ['Pending', 'Downloading', 'Verifying', 'Applying', 'Success', 'Failed', 'Rollback'];
    
    const otaUpdates = [];
    const now = Date.now();
    
    // Generate 5-15 OTA updates
    const updateCount = 5 + Math.floor(Math.random() * 11);
    
    for (let i = 0; i < updateCount; i++) {
      const target = targets[Math.floor(Math.random() * targets.length)];
      const priority = priorities[Math.floor(Math.random() * priorities.length)];
      const status = statuses[Math.floor(Math.random() * statuses.length)];
      
      // Generate realistic created time (last 30 days)
      const createdAt = new Date(now - Math.random() * 30 * 24 * 3600000).toISOString();
      
      let startedAt = null;
      let completedAt = null;
      
      if (status !== 'Pending') {
        startedAt = new Date(new Date(createdAt).getTime() + Math.random() * 3600000).toISOString();
      }
      
      if (status === 'Success' || status === 'Failed' || status === 'Rollback') {
        completedAt = new Date(new Date(startedAt || createdAt).getTime() + Math.random() * 7200000).toISOString();
      }
      
      // Select random trucks for this update (1-3 trucks or all trucks)
      const truckIds = Math.random() > 0.7 
        ? trucks.map(t => t.id) 
        : trucks
            .sort(() => 0.5 - Math.random())
            .slice(0, 1 + Math.floor(Math.random() * 3))
            .map(t => t.id);
      
      otaUpdates.push({
        id: uuidv4(),
        update_id: `UPDATE-${uuidv4().substring(0, 8)}`,
        truck_id: truckIds.length === 1 ? truckIds[0] : null,
        fleet_id: truckIds.length > 1 ? uuidv4() : null,
        version: `2.${Math.floor(Math.random() * 10)}.${Math.floor(Math.random() * 10)}`,
        target,
        url: `https://updates.example.com/${target.toLowerCase()}-${uuidv4().substring(0, 8)}.bin`,
        checksum: `sha256:deadbeef${uuidv4().substring(0, 8)}`,
        signature: `sig:${uuidv4()}`,
        size_bytes: 1024 * 1024 + Math.floor(Math.random() * 99 * 1024 * 1024),
        priority,
        requires_reboot: target === 'Firmware' || Math.random() > 0.5,
        deadline: Math.random() > 0.5 ? new Date(now + 7 * 24 * 3600000).toISOString() : null,
        meta: {
          description: this.generateUpdateDescription(target, priority),
          author: ['John Doe', 'Jane Smith', 'Bob Johnson'][Math.floor(Math.random() * 3)],
          release_notes: `Version ${uuidv4().substring(0, 8)} release notes`,
          compatibility: ['Model-X', 'Model-Y', 'Model-Z'],
          estimated_apply_time_sec: 300 + Math.floor(Math.random() * 900),
        },
        status,
        progress_percent: status === 'Success' || status === 'Failed' || status === 'Rollback' ? 100 : Math.random() * 100,
        started_at: startedAt,
        completed_at: completedAt,
        last_error: status === 'Failed' ? 'Download failed: network error' : null,
        created_at: createdAt,
        updated_at: completedAt || startedAt || createdAt,
      });
    }
    
    return otaUpdates;
  }

  // Generate update description
  generateUpdateDescription(target, priority) {
    const descriptions = {
      Agent: {
        Critical: 'Critical security update - apply immediately',
        High: 'Important bug fixes and performance improvements',
        Medium: 'New features and minor improvements',
        Low: 'Cosmetic changes and documentation updates',
      },
      Model: {
        Critical: 'Critical model update - improved accuracy and safety',
        High: 'Improved model performance and reduced false positives',
        Medium: 'Added support for new scenarios',
        Low: 'Minor model tweaks and optimizations',
      },
      Config: {
        Critical: 'Critical configuration changes - apply immediately',
        High: 'Important configuration updates for better performance',
        Medium: 'New configuration options and defaults',
        Low: 'Minor configuration tweaks',
      },
      Firmware: {
        Critical: 'Critical firmware update - apply immediately',
        High: 'Important firmware improvements and bug fixes',
        Medium: 'New firmware features and optimizations',
        Low: 'Minor firmware tweaks',
      },
    };
    
    return descriptions[target]?.[priority] || `${target} update (${priority})`;
  }

  // Generate remote commands for trucks
  generateRemoteCommands(trucks) {
    const commandTypes = [
      'Reboot', 'Shutdown', 'RestartService', 
      'GetDiagnostics', 'UpdateConfig', 'RunHealthCheck',
      'CaptureSnapshot', 'FlushWAL'
    ];
    const statuses = ['Pending', 'Executing', 'Success', 'Failed', 'Timeout', 'Cancelled'];
    
    const commands = [];
    const now = Date.now();
    
    // Generate 10-20 remote commands
    const commandCount = 10 + Math.floor(Math.random() * 11);
    
    for (let i = 0; i < commandCount; i++) {
      const commandType = commandTypes[Math.floor(Math.random() * commandTypes.length)];
      const status = statuses[Math.floor(Math.random() * statuses.length)];
      
      // Generate realistic issued time (last 7 days)
      const issuedAt = new Date(now - Math.random() * 7 * 24 * 3600000).toISOString();
      
      let completedAt = null;
      
      if (status === 'Success' || status === 'Failed' || status === 'Timeout' || status === 'Cancelled') {
        completedAt = new Date(new Date(issuedAt).getTime() + Math.random() * 3600000).toISOString();
      }
      
      // Select random trucks for this command (1-3 trucks or all trucks)
      const truckIds = Math.random() > 0.7 
        ? trucks.map(t => t.id) 
        : trucks
            .sort(() => 0.5 - Math.random())
            .slice(0, 1 + Math.floor(Math.random() * 3))
            .map(t => t.id);
      
      commands.push({
        id: uuidv4(),
        command_id: `CMD-${uuidv4().substring(0, 8)}`,
        truck_id: truckIds.length === 1 ? truckIds[0] : null,
        fleet_id: truckIds.length > 1 ? uuidv4() : null,
        command_type: commandType,
        parameters: this.generateCommandParameters(commandType),
        issued_at: issuedAt,
        deadline: Math.random() > 0.5 ? new Date(now + 24 * 3600000).toISOString() : null,
        requires_ack: Math.random() > 0.5,
        status,
        result: status === 'Success' ? this.generateCommandResult(commandType) : null,
        error: status === 'Failed' ? 'Command execution failed: timeout' : null,
        completed_at: completedAt,
        created_at: issuedAt,
        updated_at: completedAt || issuedAt,
      });
    }
    
    return commands;
  }

  // Generate command parameters
  generateCommandParameters(commandType) {
    switch (commandType) {
      case 'Reboot':
        return {
          reason: 'scheduled_maintenance',
          delay_seconds: 30,
        };
      case 'Shutdown':
        return {
          reason: 'system_update',
          delay_seconds: 60,
        };
      case 'RestartService':
        return {
          service: 'ml_engine',
          timeout_seconds: 30,
        };
      case 'GetDiagnostics':
        return {
          detail_level: 'full',
          include_logs: true,
        };
      case 'UpdateConfig':
        return {
          config: {
            ml_edge: {
              enable_drowsiness: true,
              enable_lane_departure: true,
              camera: {
                fps: 15,
              },
            },
          },
        };
      case 'RunHealthCheck':
        return {
          check_type: 'full',
          timeout_seconds: 60,
        };
      case 'CaptureSnapshot':
        return {
          include_logs: true,
          include_config: true,
        };
      case 'FlushWAL':
        return {
          force: true,
        };
      default:
        return {};
    }
  }

  // Generate command result
  generateCommandResult(commandType) {
    switch (commandType) {
      case 'Reboot':
        return {
          success: true,
          message: 'System will reboot in 30 seconds',
          reboot_time: new Date(Date.now() + 30000).toISOString(),
        };
      case 'Shutdown':
        return {
          success: true,
          message: 'System will shutdown in 60 seconds',
          shutdown_time: new Date(Date.now() + 60000).toISOString(),
        };
      case 'RestartService':
        return {
          success: true,
          message: 'Service restarted successfully',
          service: 'ml_engine',
          start_time: new Date().toISOString(),
        };
      case 'GetDiagnostics':
        return {
          success: true,
          message: 'Diagnostics collected successfully',
          diagnostics: {
            uptime: '10h',
            memory_usage: '45%',
            disk_usage: '60%',
            temperature: '45C',
          },
        };
      case 'UpdateConfig':
        return {
          success: true,
          message: 'Configuration updated successfully',
          config_version: `2.${Math.floor(Math.random() * 10)}.${Math.floor(Math.random() * 10)}`,
        };
      case 'RunHealthCheck':
        return {
          success: true,
          message: 'Health check completed successfully',
          health_score: 85 + Math.floor(Math.random() * 15),
        };
      case 'CaptureSnapshot':
        return {
          success: true,
          message: 'Snapshot captured successfully',
          snapshot_id: `SNAPSHOT-${uuidv4().substring(0, 8)}`,
          size_bytes: 1024 * 1024 + Math.floor(Math.random() * 9 * 1024 * 1024),
        };
      case 'FlushWAL':
        return {
          success: true,
          message: 'WAL flushed successfully',
          entries_flushed: 1000 + Math.floor(Math.random() * 9000),
        };
      default:
        return {
          success: true,
          message: 'Command executed successfully',
        };
    }
  }

  // Auth methods
  async login(username, password) {
    // Simulate API delay
    await this.delay(500);
    
    // Dummy auth - accept any credentials
    const token = `dummy-token-${uuidv4()}`;
    const user = {
      id: uuidv4(),
      username: username,
      name: username,
      role: 'admin',
      created_at: new Date().toISOString(),
    };
    
    localStorage.setItem('authToken', token);
    localStorage.setItem('user', JSON.stringify(user));
    
    return { token, user };
  }

  async validateToken() {
    await this.delay(200);
    
    const token = localStorage.getItem('authToken');
    const user = localStorage.getItem('user');
    
    if (!token || !user) {
      throw new Error('Invalid token');
    }
    
    return { user: JSON.parse(user) };
  }

  // Trucks methods
  async getTrucks(params = {}) {
    await this.delay(300);
    
    let trucks = JSON.parse(localStorage.getItem('trucks') || '[]');
    
    // Apply filters
    if (params.status) {
      trucks = trucks.filter(truck => truck.status === params.status);
    }
    
    if (params.make) {
      trucks = trucks.filter(truck => truck.make === params.make);
    }
    
    if (params.model) {
      trucks = trucks.filter(truck => truck.model === params.model);
    }
    
    // Apply pagination
    const page = params.page || 1;
    const limit = params.limit || 10;
    const startIndex = (page - 1) * limit;
    const endIndex = startIndex + limit;
    
    const paginatedTrucks = trucks.slice(startIndex, endIndex);
    
    return {
      data: paginatedTrucks,
      total: trucks.length,
      page,
      limit,
    };
  }

  async getTruck(id) {
    await this.delay(200);
    
    const trucks = JSON.parse(localStorage.getItem('trucks') || '[]');
    const truck = trucks.find(t => t.id === id);
    
    if (!truck) {
      throw new Error('Truck not found');
    }
    
    // Add recent telemetry
    const telemetry = JSON.parse(localStorage.getItem('telemetry') || '[]');
    const truckTelemetry = telemetry
      .filter(t => t.truck_id === id)
      .sort((a, b) => new Date(b.timestamp) - new Date(a.timestamp))
      .slice(0, 10);
    
    // Add recent alerts
    const alerts = JSON.parse(localStorage.getItem('alerts') || '[]');
    const truckAlerts = alerts
      .filter(a => a.truck_id === id)
      .sort((a, b) => new Date(b.triggered_at) - new Date(a.triggered_at))
      .slice(0, 5);
    
    // Add recent ML events
    const mlEvents = JSON.parse(localStorage.getItem('mlEvents') || '[]');
    const truckMlEvents = mlEvents
      .filter(m => m.truck_id === id)
      .sort((a, b) => new Date(b.timestamp) - new Date(a.timestamp))
      .slice(0, 5);
    
    // Add recent health status
    const healthStatus = JSON.parse(localStorage.getItem('healthStatus') || '[]');
    const truckHealthStatus = healthStatus
      .filter(h => h.truck_id === id)
      .sort((a, b) => new Date(b.timestamp) - new Date(a.timestamp))
      .slice(0, 1);
    
    // Add recent trips (simulated)
    const trips = this.generateTrips(id);
    
    // Add maintenance history (simulated)
    const maintenanceHistory = this.generateMaintenanceHistory(id);
    
    return {
      ...truck,
      sensors: truckTelemetry.length > 0 ? truckTelemetry[0].sensors : null,
      cameras: truckTelemetry.length > 0 ? truckTelemetry[0].cameras : null,
      ml_events: truckMlEvents,
      health_status: truckHealthStatus.length > 0 ? truckHealthStatus[0] : null,
      recent_trips: trips,
      maintenance_history: maintenanceHistory,
      active_alerts: truckAlerts.length,
      health_score: truckHealthStatus.length > 0 ? this.calculateHealthScore(truckHealthStatus[0]) : 85,
    };
  }

  async createTruck(truckData) {
    await this.delay(400);
    
    const trucks = JSON.parse(localStorage.getItem('trucks') || '[]');
    const newTruck = {
      id: uuidv4(),
      truck_id: `TRK-${String(trucks.length + 1).padStart(4, '0')}`,
      ...truckData,
      status: truckData.status || 'Offline',
      last_seen: new Date().toISOString(),
      location: truckData.location || [-122.4194, 37.7749],
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    
    trucks.push(newTruck);
    localStorage.setItem('trucks', JSON.stringify(trucks));
    
    // Generate some telemetry data for the new truck
    const telemetry = JSON.parse(localStorage.getItem('telemetry') || '[]');
    const newTelemetry = this.generateTelemetryForTruck(newTruck);
    telemetry.push(...newTelemetry);
    localStorage.setItem('telemetry', JSON.stringify(telemetry));
    
    return newTruck;
  }

  async updateTruck(id, truckData) {
    await this.delay(300);
    
    const trucks = JSON.parse(localStorage.getItem('trucks') || '[]');
    const index = trucks.findIndex(t => t.id === id);
    
    if (index === -1) {
      throw new Error('Truck not found');
    }
    
    trucks[index] = {
      ...trucks[index],
      ...truckData,
      updated_at: new Date().toISOString(),
    };
    
    localStorage.setItem('trucks', JSON.stringify(trucks));
    return trucks[index];
  }

  async deleteTruck(id) {
    await this.delay(300);
    
    let trucks = JSON.parse(localStorage.getItem('trucks') || '[]');
    const truckIndex = trucks.findIndex(t => t.id === id);
    
    if (truckIndex === -1) {
      throw new Error('Truck not found');
    }
    
    trucks = trucks.filter(t => t.id !== id);
    localStorage.setItem('trucks', JSON.stringify(trucks));
    
    // Also remove related data
    let telemetry = JSON.parse(localStorage.getItem('telemetry') || '[]');
    telemetry = telemetry.filter(t => t.truck_id !== id);
    localStorage.setItem('telemetry', JSON.stringify(telemetry));
    
    let alerts = JSON.parse(localStorage.getItem('alerts') || '[]');
    alerts = alerts.filter(a => a.truck_id !== id);
    localStorage.setItem('alerts', JSON.stringify(alerts));
    
    let mlEvents = JSON.parse(localStorage.getItem('mlEvents') || '[]');
    mlEvents = mlEvents.filter(m => m.truck_id !== id);
    localStorage.setItem('mlEvents', JSON.stringify(mlEvents));
    
    let healthStatus = JSON.parse(localStorage.getItem('healthStatus') || '[]');
    healthStatus = healthStatus.filter(h => h.truck_id !== id);
    localStorage.setItem('healthStatus', JSON.stringify(healthStatus));
    
    return { success: true };
  }

  // Telemetry methods
  async getTruckTelemetry(truckId, params = {}) {
    await this.delay(200);
    
    let telemetry = JSON.parse(localStorage.getItem('telemetry') || '[]');
    telemetry = telemetry.filter(t => t.truck_id === truckId);
    
    // Apply date range filter
    if (params.startDate) {
      telemetry = telemetry.filter(t => new Date(t.timestamp) >= new Date(params.startDate));
    }
    
    if (params.endDate) {
      telemetry = telemetry.filter(t => new Date(t.timestamp) <= new Date(params.endDate));
    }
    
    // Apply pagination
    const page = params.page || 1;
    const limit = params.limit || 50;
    const startIndex = (page - 1) * limit;
    const endIndex = startIndex + limit;
    
    const paginatedTelemetry = telemetry.slice(startIndex, endIndex);
    
    return {
      data: paginatedTelemetry,
      total: telemetry.length,
      page,
      limit,
    };
  }

  // Alerts methods
  async getAlerts(params = {}) {
    await this.delay(200);
    
    let alerts = JSON.parse(localStorage.getItem('alerts') || '[]');
    
    // Apply filters
    if (params.severity) {
      alerts = alerts.filter(a => a.severity === params.severity);
    }
    
    if (params.alertType) {
      alerts = alerts.filter(a => a.alert_type === params.alertType);
    }
    
    if (params.status) {
      alerts = alerts.filter(a => a