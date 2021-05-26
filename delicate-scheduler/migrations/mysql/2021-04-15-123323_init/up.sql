CREATE TABLE `task` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
  `name` varchar(128) NOT NULL COMMENT 'Task name',
  `description` varchar(128) NOT NULL COMMENT 'Task description',
  `command` varchar(256) NOT NULL COMMENT 'Task execute command',
  `frequency` varchar(256) NOT NULL COMMENT 'Task frequency',
  `cron_expression` varchar(256) NOT NULL COMMENT 'Task cron expression',
  `timeout` smallint(9) NOT NULL DEFAULT '0' COMMENT 'Task Timeout',
  `retry_times` smallint(6) NOT NULL DEFAULT '0' COMMENT 'Task retry times',
  `retry_interval` smallint(6) NOT NULL DEFAULT '0' COMMENT 'Task retest interval',
  `maximun_parallel_runnable_num` smallint(11) NOT NULL DEFAULT '0' COMMENT 'Maximum number of parallel tasks',
  `tag` varchar(32) NOT NULL DEFAULT '' COMMENT 'Task tag',
  `status` smallint(6) NOT NULL DEFAULT '1' COMMENT 'Task status',
  `created_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Task creation time',
  `deleted_time` timestamp NULL DEFAULT NULL COMMENT 'Task deletion time',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `task_log` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'Run record id generated by the task in the executor',
  `task_id` bigint(20) NOT NULL DEFAULT '0' COMMENT 'Task-id',
  `name` varchar(128) NOT NULL COMMENT 'Task name (snapshot)',
  `description` varchar(128) NOT NULL COMMENT 'Task description (snapshot)',
  `command` varchar(256) NOT NULL COMMENT 'Task command (snapshot)',
  `frequency` varchar(256) NOT NULL COMMENT 'Task frequency (snapshot)',
  `cron_expression` varchar(256) NOT NULL COMMENT 'Task cron-expression (snapshot)',
  `maximun_parallel_runnable_num` smallint(11) NOT NULL DEFAULT '0' COMMENT 'Maximum number of parallel tasks(snapshot)',
  `tag` varchar(32) NOT NULL DEFAULT '' COMMENT 'Task tag (snapshot)',
  `status` smallint(6) NOT NULL DEFAULT '1' COMMENT 'Status',
  `created_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Task log creation time',
  `updated_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'Update Task log time',
  `executor_processor_id` bigint(20) NOT NULL COMMENT 'Executor processor id',
  `executor_processor_name` varchar(255) NOT NULL COMMENT 'Executor processor name (snapshot)',
  `executor_processor_host` varchar(32) NOT NULL DEFAULT '' COMMENT 'Executor processor host',
  PRIMARY KEY (`id`),
  KEY `task_id_idx` (`task_id`) USING BTREE,
  KEY `executor_processor_id_idx` (`executor_processor_id`) USING BTREE,
  KEY `created_time_idx` (`created_time`) USING BTREE,
  KEY `updated_time_idx` (`updated_time`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `task_log_extend` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'Run record id generated by the task in the executor',
  `task_id` bigint(20) NOT NULL DEFAULT '0' COMMENT 'Task-id',
 	`stdout` text NOT NULL COMMENT 'process-child-stdout or http response',
	`stderr` text NOT NULL COMMENT 'process-child-stderr',
  PRIMARY KEY (`id`),
  KEY `task_id_idx` (`task_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


CREATE TABLE `executor_processor` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
  `name` varchar(128) NOT NULL COMMENT 'Executor-processor name',
  `host` varchar(32) NOT NULL DEFAULT '' COMMENT 'Executor-processor host',
  `port` smallint(6) NOT NULL DEFAULT '0' COMMENT 'Executor-processor port',
  `machine_id` smallint(6) NOT NULL DEFAULT '0' COMMENT 'Unique machine id of the executor, Max is 1024',
  `description` varchar(128) NOT NULL COMMENT 'Executor-processor description',
  `tag` varchar(32) NOT NULL DEFAULT '' COMMENT 'Executor-processor tag',
  `created_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Executor-processor creation time',
  `deleted_time` timestamp NULL DEFAULT NULL COMMENT 'Executor-processor delition time',
  PRIMARY KEY (`id`),
  UNIQUE KEY `machine_id_idx` (`machine_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `executor_group` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
  `name` varchar(128) NOT NULL COMMENT 'Executor-group name',
  `description` varchar(128) NOT NULL COMMENT 'Executor-group description',
  `tag` varchar(32) NOT NULL DEFAULT '' COMMENT 'Executor-group tag',
  `created_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Executor-group creation time',
  `deleted_time` timestamp NULL DEFAULT NULL COMMENT 'Executor-group delition time',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `executor_processor_bind` (
`id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
`name` varchar(128) NOT NULL COMMENT 'Executor-group-processor name',
`group_id` bigint(20) NOT NULL DEFAULT '0' COMMENT 'Executor group id',
`executor_id` bigint(20) NOT NULL DEFAULT '0' COMMENT 'Executor id',
`weight` smallint(11) NOT NULL DEFAULT '0' COMMENT 'Execution weights of tasks between executor in a group',
`created_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Executor-processor-group creation time',
PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;

CREATE TABLE `task_bind` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
  `task_id` bigint(20) NOT NULL DEFAULT '0' COMMENT 'Task-id',
  `bind_id` bigint(20) NOT NULL DEFAULT '0' COMMENT 'Bind-id (executor_processor_group_id)',
  `created_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Binding creation time',
  PRIMARY KEY (`id`),
  KEY `task_id_idx` (`task_id`) USING BTREE,
  KEY `bind_id_idx` (`bind_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `user_auth` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `user_id` bigint(20) unsigned NOT NULL DEFAULT '0' COMMENT 'user-id',
  `identity_type` tinyint(4) unsigned NOT NULL DEFAULT '1' COMMENT '1:Mobie-number 2:Email 3:Username 4:LDAP 5:Other-OAuth',
  `identifier` varchar(32) NOT NULL DEFAULT '' COMMENT 'Mobie-number Email Username or unique identifier for third party applications',
  `certificate` varchar(32) NOT NULL DEFAULT '' COMMENT 'Password credentials (the station saves the password, the station does not save or save the token)',
  `status` tinyint(4) NOT NULL DEFAULT '1' COMMENT 'Authentication status: 1:active, 2:disabled',
  `created_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Binding time',
  `updated_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'Update binding time',
  PRIMARY KEY (`id`),
  UNIQUE KEY `only` (`user_id`,`identity_type`),
  UNIQUE KEY `identifier_idx` (`identifier`) USING BTREE,
  KEY `user_id_idx` (`user_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='User authorization';

CREATE TABLE `user` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT 'user-id',
  `user_name` varchar(32) NOT NULL DEFAULT '' COMMENT 'User account, must be unique',
  `nick_name` varchar(32) NOT NULL DEFAULT '' COMMENT 'User nickname',
  `mobile` varchar(16) NOT NULL DEFAULT '' COMMENT 'Mobie-number(unique)',
  `email` varchar(100) NOT NULL DEFAULT '' COMMENT 'Email(unique)',
  `face` varchar(255) NOT NULL DEFAULT '' COMMENT 'Avatar',
  `status` tinyint(4) NOT NULL DEFAULT '1' COMMENT 'User Status: 2:Deleted',
  `created_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Creation time',
  `updated_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'Modify time',
  PRIMARY KEY (`id`),
  UNIQUE KEY `user_name_idx` (`user_name`) USING BTREE COMMENT 'User name, must be unique',
  UNIQUE KEY `user_mobile_idx` (`mobile`) USING BTREE COMMENT 'Mobie-number(unique)',
  UNIQUE KEY `user_email_idx` (`email`) USING BTREE COMMENT 'Email(unique)'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='User Base Information';

CREATE TABLE `user_login_log` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `user_id` bigint(20) unsigned NOT NULL DEFAULT '0' COMMENT 'user-id',
  `login_type` tinyint(3) unsigned NOT NULL DEFAULT '1' COMMENT 'Login method Third party/email/mobile etc.',
  `command` tinyint(3) unsigned NOT NULL DEFAULT '1' COMMENT 'Operation type 1:Login success 2:Logout success 3:Login failure 4:Logout failure',
  `lastip` varchar(32) NOT NULL DEFAULT '' COMMENT 'Login-ip',
  `created_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Creation time',
  PRIMARY KEY (`id`),
  KEY `idx_user_id_type_time` (`user_id`,`login_type`,`created_time`) USING BTREE,
  KEY `idx_create_time` (`created_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='Login Log';


CREATE TABLE `user_register_log` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
  `user_id` bigint(20) unsigned NOT NULL COMMENT 'user-id',
  `register_method` tinyint(2) unsigned NOT NULL COMMENT 'Registration method 1:Mobie-number 2:Email 3:Username',
  `register_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Registration Time',
  `register_ip` varchar(32) NOT NULL DEFAULT '' COMMENT 'Registered IP',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb4 COMMENT='User Registration Log';


CREATE TABLE `user_info_update` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'Self-incrementing id',
  `user_id` bigint(20) unsigned NOT NULL COMMENT 'user-id',
  `attribute_name` varchar(30) NOT NULL COMMENT 'Attribute Name',
  `attribute_old_val` varchar(30) NOT NULL DEFAULT '' COMMENT 'attribute corresponds to the old value',
  `attribute_new_val` varchar(30) NOT NULL DEFAULT '' COMMENT 'attribute corresponds to the new value',
  `updated_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'Modify time',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb4 COMMENT='User Modification Information';
