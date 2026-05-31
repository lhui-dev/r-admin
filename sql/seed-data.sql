-- Seed data for Rust + PostgreSQL + Vue3 RBAC admin system
-- This script is designed to be idempotent as much as possible.
--
-- Important:
-- Default plaintext password: Admin@123456
-- The value below is an Argon2id PHC string for the default admin password.

BEGIN;

-- 1. Departments
INSERT INTO sys_dept (
    id, parent_id, dept_name, dept_code, leader_user_id, sort_no, status,
    created_at, updated_at, created_by, updated_by, is_deleted, remark
) VALUES
    (0, 0, 'ROOT', 'ROOT', NULL, 0, 1, NOW(), NOW(), 1000, 1000, FALSE, '系统虚拟根部门'),
    (100, 0, '平台总部', 'HQ', NULL, 1, 1, NOW(), NOW(), 1000, 1000, FALSE, '系统初始化根部门'),
    (110, 100, '技术中心', 'TECH', NULL, 10, 1, NOW(), NOW(), 1000, 1000, FALSE, '研发与平台技术部门'),
    (120, 100, '运营中心', 'OPS', NULL, 20, 1, NOW(), NOW(), 1000, 1000, FALSE, '运营管理部门')
ON CONFLICT (id) DO UPDATE SET
    parent_id = EXCLUDED.parent_id,
    dept_name = EXCLUDED.dept_name,
    dept_code = EXCLUDED.dept_code,
    sort_no = EXCLUDED.sort_no,
    status = EXCLUDED.status,
    updated_at = NOW(),
    updated_by = EXCLUDED.updated_by,
    is_deleted = FALSE,
    remark = EXCLUDED.remark;

-- 2. Posts
INSERT INTO sys_post (
    id, post_name, post_code, sort_no, status,
    created_at, updated_at, created_by, updated_by, is_deleted, remark
) VALUES
    (200, '超级管理员', 'SUPER_ADMIN', 1, 1, NOW(), NOW(), 1000, 1000, FALSE, '系统内置岗位'),
    (210, '系统管理员', 'SYSTEM_ADMIN', 10, 1, NOW(), NOW(), 1000, 1000, FALSE, '系统管理岗位'),
    (220, '审计员', 'AUDITOR', 20, 1, NOW(), NOW(), 1000, 1000, FALSE, '审计查看岗位')
ON CONFLICT (id) DO UPDATE SET
    post_name = EXCLUDED.post_name,
    post_code = EXCLUDED.post_code,
    sort_no = EXCLUDED.sort_no,
    status = EXCLUDED.status,
    updated_at = NOW(),
    updated_by = EXCLUDED.updated_by,
    is_deleted = FALSE,
    remark = EXCLUDED.remark;

-- 3. Users
INSERT INTO sys_user (
    id, username, password_hash, nickname, real_name, mobile, email, avatar_url,
    gender, dept_id, status, is_super_admin, last_login_at, last_login_ip,
    password_updated_at, created_at, updated_at, created_by, updated_by, is_deleted, remark
) VALUES
    (
        1000,
        'admin',
        '$argon2id$v=19$m=19456,t=2,p=1$ci1hZG1pbi1pbml0LXNhbHQ$KmrVD0vQzQRZwFYNpBXzOHGqos4qki+I7JajPBznjdI',
        '超级管理员',
        '平台管理员',
        '13800000000',
        'admin@example.com',
        NULL,
        1,
        100,
        1,
        TRUE,
        NULL,
        NULL,
        NOW(),
        NOW(),
        NOW(),
        1000,
        1000,
        FALSE,
        '系统初始化管理员账号'
    )
ON CONFLICT (username) DO UPDATE SET
    nickname = EXCLUDED.nickname,
    real_name = EXCLUDED.real_name,
    mobile = EXCLUDED.mobile,
    email = EXCLUDED.email,
    dept_id = EXCLUDED.dept_id,
    status = EXCLUDED.status,
    is_super_admin = EXCLUDED.is_super_admin,
    updated_at = NOW(),
    updated_by = EXCLUDED.updated_by,
    is_deleted = FALSE,
    remark = EXCLUDED.remark;

UPDATE sys_dept
SET leader_user_id = 1000, updated_at = NOW(), updated_by = 1000
WHERE id = 100;

-- 4. Roles
INSERT INTO sys_role (
    id, role_name, role_code, role_sort, data_scope, status, is_builtin,
    created_at, updated_at, created_by, updated_by, is_deleted, remark
) VALUES
    (300, '超级管理员', 'super_admin', 1, 'ALL', 1, TRUE, NOW(), NOW(), 1000, 1000, FALSE, '系统内置超级管理员角色'),
    (310, '系统管理员', 'system_admin', 10, 'DEPT_AND_CHILD', 1, TRUE, NOW(), NOW(), 1000, 1000, FALSE, '系统内置系统管理员角色'),
    (320, '审计员', 'auditor', 20, 'ALL', 1, TRUE, NOW(), NOW(), 1000, 1000, FALSE, '系统内置审计角色'),
    (330, '普通操作员', 'operator', 30, 'SELF', 1, TRUE, NOW(), NOW(), 1000, 1000, FALSE, '系统内置普通操作角色')
ON CONFLICT (role_code) DO UPDATE SET
    role_name = EXCLUDED.role_name,
    role_sort = EXCLUDED.role_sort,
    data_scope = EXCLUDED.data_scope,
    status = EXCLUDED.status,
    is_builtin = EXCLUDED.is_builtin,
    updated_at = NOW(),
    updated_by = EXCLUDED.updated_by,
    is_deleted = FALSE,
    remark = EXCLUDED.remark;

-- 5. Permissions
INSERT INTO sys_permission (
    id, permission_name, permission_code, permission_type, http_method, api_path,
    status, created_at, updated_at, created_by, updated_by, is_deleted, remark
) VALUES
    (10000, '首页查看', 'dashboard:view', 'menu', 'GET', '/api/dashboard/overview', 1, NOW(), NOW(), 1000, 1000, FALSE, '首页概览'),

    (10100, '用户列表', 'system:user:list', 'menu', 'GET', '/api/system/users', 1, NOW(), NOW(), 1000, 1000, FALSE, '查看用户列表'),
    (10101, '用户新增', 'system:user:create', 'button', 'POST', '/api/system/users', 1, NOW(), NOW(), 1000, 1000, FALSE, '新增用户'),
    (10102, '用户修改', 'system:user:update', 'button', 'PUT', '/api/system/users/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '修改用户'),
    (10103, '用户删除', 'system:user:delete', 'button', 'DELETE', '/api/system/users/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '删除用户'),
    (10104, '用户重置密码', 'system:user:reset-password', 'button', 'PUT', '/api/system/users/:id/reset-password', 1, NOW(), NOW(), 1000, 1000, FALSE, '重置密码'),
    (10105, '用户分配角色', 'system:user:assign-role', 'button', 'PUT', '/api/system/users/:id/roles', 1, NOW(), NOW(), 1000, 1000, FALSE, '分配角色'),

    (10200, '角色列表', 'system:role:list', 'menu', 'GET', '/api/system/roles', 1, NOW(), NOW(), 1000, 1000, FALSE, '查看角色列表'),
    (10201, '角色新增', 'system:role:create', 'button', 'POST', '/api/system/roles', 1, NOW(), NOW(), 1000, 1000, FALSE, '新增角色'),
    (10202, '角色修改', 'system:role:update', 'button', 'PUT', '/api/system/roles/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '修改角色'),
    (10203, '角色删除', 'system:role:delete', 'button', 'DELETE', '/api/system/roles/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '删除角色'),
    (10204, '角色分配权限', 'system:role:assign-permission', 'button', 'PUT', '/api/system/roles/:id/permissions', 1, NOW(), NOW(), 1000, 1000, FALSE, '分配权限'),

    (10300, '菜单列表', 'system:menu:list', 'menu', 'GET', '/api/system/menus/tree', 1, NOW(), NOW(), 1000, 1000, FALSE, '查看菜单树'),
    (10301, '菜单新增', 'system:menu:create', 'button', 'POST', '/api/system/menus', 1, NOW(), NOW(), 1000, 1000, FALSE, '新增菜单'),
    (10302, '菜单修改', 'system:menu:update', 'button', 'PUT', '/api/system/menus/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '修改菜单'),
    (10303, '菜单删除', 'system:menu:delete', 'button', 'DELETE', '/api/system/menus/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '删除菜单'),

    (10400, '部门列表', 'system:dept:list', 'menu', 'GET', '/api/system/depts/tree', 1, NOW(), NOW(), 1000, 1000, FALSE, '查看部门树'),
    (10401, '部门新增', 'system:dept:create', 'button', 'POST', '/api/system/depts', 1, NOW(), NOW(), 1000, 1000, FALSE, '新增部门'),
    (10402, '部门修改', 'system:dept:update', 'button', 'PUT', '/api/system/depts/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '修改部门'),
    (10403, '部门删除', 'system:dept:delete', 'button', 'DELETE', '/api/system/depts/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '删除部门'),

    (10500, '岗位列表', 'system:post:list', 'menu', 'GET', '/api/system/posts', 1, NOW(), NOW(), 1000, 1000, FALSE, '查看岗位列表'),
    (10501, '岗位新增', 'system:post:create', 'button', 'POST', '/api/system/posts', 1, NOW(), NOW(), 1000, 1000, FALSE, '新增岗位'),
    (10502, '岗位修改', 'system:post:update', 'button', 'PUT', '/api/system/posts/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '修改岗位'),
    (10503, '岗位删除', 'system:post:delete', 'button', 'DELETE', '/api/system/posts/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '删除岗位'),

    (10600, '字典列表', 'system:dict:list', 'menu', 'GET', '/api/system/dicts', 1, NOW(), NOW(), 1000, 1000, FALSE, '查看字典'),
    (10601, '字典新增', 'system:dict:create', 'button', 'POST', '/api/system/dicts', 1, NOW(), NOW(), 1000, 1000, FALSE, '新增字典'),
    (10602, '字典修改', 'system:dict:update', 'button', 'PUT', '/api/system/dicts/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '修改字典'),
    (10603, '字典删除', 'system:dict:delete', 'button', 'DELETE', '/api/system/dicts/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '删除字典'),

    (10700, '参数列表', 'system:config:list', 'menu', 'GET', '/api/system/configs', 1, NOW(), NOW(), 1000, 1000, FALSE, '查看系统参数'),
    (10701, '参数新增', 'system:config:create', 'button', 'POST', '/api/system/configs', 1, NOW(), NOW(), 1000, 1000, FALSE, '新增参数'),
    (10702, '参数修改', 'system:config:update', 'button', 'PUT', '/api/system/configs/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '修改参数'),
    (10703, '参数删除', 'system:config:delete', 'button', 'DELETE', '/api/system/configs/:id', 1, NOW(), NOW(), 1000, 1000, FALSE, '删除参数'),

    (10800, '登录日志列表', 'system:log:login:list', 'menu', 'GET', '/api/system/logins', 1, NOW(), NOW(), 1000, 1000, FALSE, '查看登录日志'),
    (10810, '操作日志列表', 'system:log:operation:list', 'menu', 'GET', '/api/system/operations', 1, NOW(), NOW(), 1000, 1000, FALSE, '查看操作日志')
ON CONFLICT (permission_code) DO UPDATE SET
    permission_name = EXCLUDED.permission_name,
    permission_type = EXCLUDED.permission_type,
    http_method = EXCLUDED.http_method,
    api_path = EXCLUDED.api_path,
    status = EXCLUDED.status,
    updated_at = NOW(),
    updated_by = EXCLUDED.updated_by,
    is_deleted = FALSE,
    remark = EXCLUDED.remark;

-- 6. Menus
INSERT INTO sys_menu (
    id, parent_id, menu_name, menu_type, route_name, route_path, component_path,
    permission_code, icon, sort_no, visible, keep_alive, is_external, status,
    created_at, updated_at, created_by, updated_by, is_deleted, remark
) VALUES
    (20000, 0, '首页', 'catalog', 'DashboardRoot', '/dashboard', NULL, NULL, 'House', 1, TRUE, FALSE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '首页目录'),
    (20001, 20000, '工作台', 'menu', 'DashboardWorkbench', '/dashboard/workbench', 'dashboard/workbench/index', 'dashboard:view', 'Monitor', 1, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '工作台首页'),

    (21000, 0, '系统管理', 'catalog', 'SystemRoot', '/system', NULL, NULL, 'Setting', 10, TRUE, FALSE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '系统管理目录'),
    (21010, 21000, '用户管理', 'menu', 'SystemUser', '/system/user', 'system/user/index', 'system:user:list', 'User', 10, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '用户管理页面'),
    (21020, 21000, '角色管理', 'menu', 'SystemRole', '/system/role', 'system/role/index', 'system:role:list', 'UserFilled', 20, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '角色管理页面'),
    (21030, 21000, '菜单管理', 'menu', 'SystemMenu', '/system/menu', 'system/menu/index', 'system:menu:list', 'Menu', 30, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '菜单管理页面'),
    (21040, 21000, '部门管理', 'menu', 'SystemDept', '/system/dept', 'system/dept/index', 'system:dept:list', 'OfficeBuilding', 40, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '部门管理页面'),
    (21050, 21000, '岗位管理', 'menu', 'SystemPost', '/system/post', 'system/post/index', 'system:post:list', 'Suitcase', 50, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '岗位管理页面'),
    (21060, 21000, '字典管理', 'menu', 'SystemDict', '/system/dict', 'system/dict/index', 'system:dict:list', 'CollectionTag', 60, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '字典管理页面'),
    (21070, 21000, '参数配置', 'menu', 'SystemConfig', '/system/config', 'system/config/index', 'system:config:list', 'Tools', 70, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '参数配置页面'),

    (22000, 0, '日志审计', 'catalog', 'AuditRoot', '/audit', NULL, NULL, 'Document', 20, TRUE, FALSE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '日志审计目录'),
    (22010, 22000, '登录日志', 'menu', 'AuditLoginLog', '/audit/login-log', 'audit/login-log/index', 'system:log:login:list', 'Tickets', 10, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '登录日志页面'),
    (22020, 22000, '操作日志', 'menu', 'AuditOperationLog', '/audit/operation-log', 'audit/operation-log/index', 'system:log:operation:list', 'Notebook', 20, TRUE, TRUE, FALSE, 1, NOW(), NOW(), 1000, 1000, FALSE, '操作日志页面')
ON CONFLICT (id) DO UPDATE SET
    parent_id = EXCLUDED.parent_id,
    menu_name = EXCLUDED.menu_name,
    menu_type = EXCLUDED.menu_type,
    route_name = EXCLUDED.route_name,
    route_path = EXCLUDED.route_path,
    component_path = EXCLUDED.component_path,
    permission_code = EXCLUDED.permission_code,
    icon = EXCLUDED.icon,
    sort_no = EXCLUDED.sort_no,
    visible = EXCLUDED.visible,
    keep_alive = EXCLUDED.keep_alive,
    is_external = EXCLUDED.is_external,
    status = EXCLUDED.status,
    updated_at = NOW(),
    updated_by = EXCLUDED.updated_by,
    is_deleted = FALSE,
    remark = EXCLUDED.remark;

-- 7. User-Role relations
INSERT INTO sys_user_role (id, user_id, role_id, created_at, created_by)
VALUES
    (30000, 1000, 300, NOW(), 1000)
ON CONFLICT (user_id, role_id) DO NOTHING;

-- 8. User-Post relations
INSERT INTO sys_user_post (id, user_id, post_id, created_at, created_by)
VALUES
    (31000, 1000, 200, NOW(), 1000)
ON CONFLICT (user_id, post_id) DO NOTHING;

-- 9. Role-Permission relations
-- super_admin gets all permissions
INSERT INTO sys_role_permission (id, role_id, permission_id, created_at, created_by)
SELECT
    40000 + ROW_NUMBER() OVER (ORDER BY p.id),
    300,
    p.id,
    NOW(),
    1000
FROM sys_permission p
WHERE NOT EXISTS (
    SELECT 1
    FROM sys_role_permission rp
    WHERE rp.role_id = 300 AND rp.permission_id = p.id
);

-- system_admin gets core management permissions, excluding delete audit tables because they are view-only by design
INSERT INTO sys_role_permission (id, role_id, permission_id, created_at, created_by)
SELECT
    41000 + ROW_NUMBER() OVER (ORDER BY p.id),
    310,
    p.id,
    NOW(),
    1000
FROM sys_permission p
WHERE p.permission_code IN (
    'dashboard:view',
    'system:user:list', 'system:user:create', 'system:user:update', 'system:user:reset-password', 'system:user:assign-role',
    'system:role:list', 'system:role:create', 'system:role:update', 'system:role:assign-permission',
    'system:menu:list', 'system:menu:create', 'system:menu:update',
    'system:dept:list', 'system:dept:create', 'system:dept:update',
    'system:post:list', 'system:post:create', 'system:post:update',
    'system:dict:list', 'system:dict:create', 'system:dict:update',
    'system:config:list', 'system:config:create', 'system:config:update',
    'system:log:login:list', 'system:log:operation:list'
)
AND NOT EXISTS (
    SELECT 1
    FROM sys_role_permission rp
    WHERE rp.role_id = 310 AND rp.permission_id = p.id
);

-- auditor gets dashboard and logs
INSERT INTO sys_role_permission (id, role_id, permission_id, created_at, created_by)
SELECT
    42000 + ROW_NUMBER() OVER (ORDER BY p.id),
    320,
    p.id,
    NOW(),
    1000
FROM sys_permission p
WHERE p.permission_code IN (
    'dashboard:view',
    'system:log:login:list',
    'system:log:operation:list'
)
AND NOT EXISTS (
    SELECT 1
    FROM sys_role_permission rp
    WHERE rp.role_id = 320 AND rp.permission_id = p.id
);

-- operator gets only dashboard
INSERT INTO sys_role_permission (id, role_id, permission_id, created_at, created_by)
SELECT
    43000 + ROW_NUMBER() OVER (ORDER BY p.id),
    330,
    p.id,
    NOW(),
    1000
FROM sys_permission p
WHERE p.permission_code IN ('dashboard:view')
AND NOT EXISTS (
    SELECT 1
    FROM sys_role_permission rp
    WHERE rp.role_id = 330 AND rp.permission_id = p.id
);

-- 10. Dictionary types
INSERT INTO sys_dict_type (
    id, dict_name, dict_code, status, created_at, updated_at,
    created_by, updated_by, is_deleted, remark
) VALUES
    (50000, '通用状态', 'sys_common_status', 1, NOW(), NOW(), 1000, 1000, FALSE, '启用禁用状态'),
    (50010, '用户性别', 'sys_gender', 1, NOW(), NOW(), 1000, 1000, FALSE, '用户性别字典'),
    (50020, '菜单类型', 'sys_menu_type', 1, NOW(), NOW(), 1000, 1000, FALSE, '目录菜单按钮类型')
ON CONFLICT (dict_code) DO UPDATE SET
    dict_name = EXCLUDED.dict_name,
    status = EXCLUDED.status,
    updated_at = NOW(),
    updated_by = EXCLUDED.updated_by,
    is_deleted = FALSE,
    remark = EXCLUDED.remark;

-- 11. Dictionary items
INSERT INTO sys_dict_item (
    id, dict_type_id, item_label, item_value, item_color, sort_no, status, is_default,
    created_at, updated_at, created_by, updated_by, is_deleted, remark
) VALUES
    (51000, 50000, '启用', '1', 'success', 1, 1, TRUE, NOW(), NOW(), 1000, 1000, FALSE, '启用状态'),
    (51001, 50000, '禁用', '0', 'danger', 2, 1, FALSE, NOW(), NOW(), 1000, 1000, FALSE, '禁用状态'),

    (51010, 50010, '未知', '0', 'info', 1, 1, TRUE, NOW(), NOW(), 1000, 1000, FALSE, '未知性别'),
    (51011, 50010, '男', '1', 'primary', 2, 1, FALSE, NOW(), NOW(), 1000, 1000, FALSE, '男性'),
    (51012, 50010, '女', '2', 'danger', 3, 1, FALSE, NOW(), NOW(), 1000, 1000, FALSE, '女性'),

    (51020, 50020, '目录', 'catalog', 'warning', 1, 1, TRUE, NOW(), NOW(), 1000, 1000, FALSE, '目录'),
    (51021, 50020, '菜单', 'menu', 'primary', 2, 1, FALSE, NOW(), NOW(), 1000, 1000, FALSE, '菜单'),
    (51022, 50020, '按钮', 'button', 'success', 3, 1, FALSE, NOW(), NOW(), 1000, 1000, FALSE, '按钮')
ON CONFLICT (dict_type_id, item_value) DO UPDATE SET
    item_label = EXCLUDED.item_label,
    item_color = EXCLUDED.item_color,
    sort_no = EXCLUDED.sort_no,
    status = EXCLUDED.status,
    is_default = EXCLUDED.is_default,
    updated_at = NOW(),
    updated_by = EXCLUDED.updated_by,
    is_deleted = FALSE,
    remark = EXCLUDED.remark;

-- 12. System configs
INSERT INTO sys_config (
    id, config_name, config_key, config_value, value_type, is_builtin, status,
    created_at, updated_at, created_by, updated_by, is_deleted, remark
) VALUES
    (60000, '登录失败最大重试次数', 'security.login.max_retry', '5', 'number', TRUE, 1, NOW(), NOW(), 1000, 1000, FALSE, '超过次数可锁定账号'),
    (60001, '账号锁定分钟数', 'security.login.lock_minutes', '30', 'number', TRUE, 1, NOW(), NOW(), 1000, 1000, FALSE, '登录失败后的锁定时长'),
    (60002, '密码最小长度', 'security.password.min_length', '8', 'number', TRUE, 1, NOW(), NOW(), 1000, 1000, FALSE, '密码长度策略'),
    (60003, '默认时区', 'system.default.timezone', 'Asia/Shanghai', 'string', TRUE, 1, NOW(), NOW(), 1000, 1000, FALSE, '系统默认时区')
ON CONFLICT (config_key) DO UPDATE SET
    config_name = EXCLUDED.config_name,
    config_value = EXCLUDED.config_value,
    value_type = EXCLUDED.value_type,
    is_builtin = EXCLUDED.is_builtin,
    status = EXCLUDED.status,
    updated_at = NOW(),
    updated_by = EXCLUDED.updated_by,
    is_deleted = FALSE,
    remark = EXCLUDED.remark;

COMMIT;
