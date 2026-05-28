-- PostgreSQL schema for Rust + PostgreSQL + Vue3 RBAC admin system
-- Recommended extension (optional for fuzzy search scenarios)
-- CREATE EXTENSION IF NOT EXISTS pg_trgm;

BEGIN;

CREATE TABLE IF NOT EXISTS sys_dept (
    id BIGINT PRIMARY KEY,
    parent_id BIGINT NOT NULL DEFAULT 0,
    dept_name VARCHAR(64) NOT NULL,
    dept_code VARCHAR(64) UNIQUE,
    leader_user_id BIGINT,
    sort_no INTEGER NOT NULL DEFAULT 0,
    status SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    updated_by BIGINT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    remark VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS sys_post (
    id BIGINT PRIMARY KEY,
    post_name VARCHAR(64) NOT NULL,
    post_code VARCHAR(64) NOT NULL UNIQUE,
    sort_no INTEGER NOT NULL DEFAULT 0,
    status SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    updated_by BIGINT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    remark VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS sys_user (
    id BIGINT PRIMARY KEY,
    username VARCHAR(64) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    nickname VARCHAR(64) NOT NULL,
    real_name VARCHAR(64),
    mobile VARCHAR(32) UNIQUE,
    email VARCHAR(128) UNIQUE,
    avatar_url VARCHAR(255),
    gender SMALLINT,
    dept_id BIGINT,
    status SMALLINT NOT NULL DEFAULT 1,
    is_super_admin BOOLEAN NOT NULL DEFAULT FALSE,
    last_login_at TIMESTAMPTZ,
    last_login_ip VARCHAR(64),
    password_updated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    updated_by BIGINT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    remark VARCHAR(500),
    CONSTRAINT fk_sys_user_dept
        FOREIGN KEY (dept_id) REFERENCES sys_dept(id)
);

CREATE TABLE IF NOT EXISTS sys_role (
    id BIGINT PRIMARY KEY,
    role_name VARCHAR(64) NOT NULL,
    role_code VARCHAR(64) NOT NULL UNIQUE,
    role_sort INTEGER NOT NULL DEFAULT 0,
    data_scope VARCHAR(32),
    status SMALLINT NOT NULL DEFAULT 1,
    is_builtin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    updated_by BIGINT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    remark VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS sys_permission (
    id BIGINT PRIMARY KEY,
    permission_name VARCHAR(128) NOT NULL,
    permission_code VARCHAR(128) NOT NULL UNIQUE,
    permission_type VARCHAR(32) NOT NULL,
    http_method VARCHAR(16),
    api_path VARCHAR(255),
    status SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    updated_by BIGINT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    remark VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS sys_menu (
    id BIGINT PRIMARY KEY,
    parent_id BIGINT NOT NULL DEFAULT 0,
    menu_name VARCHAR(64) NOT NULL,
    menu_type VARCHAR(16) NOT NULL,
    route_name VARCHAR(64),
    route_path VARCHAR(255),
    component_path VARCHAR(255),
    permission_code VARCHAR(128),
    icon VARCHAR(64),
    sort_no INTEGER NOT NULL DEFAULT 0,
    visible BOOLEAN NOT NULL DEFAULT TRUE,
    keep_alive BOOLEAN NOT NULL DEFAULT FALSE,
    is_external BOOLEAN NOT NULL DEFAULT FALSE,
    status SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    updated_by BIGINT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    remark VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS sys_user_role (
    id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    CONSTRAINT uk_sys_user_role_user_role UNIQUE (user_id, role_id),
    CONSTRAINT fk_sys_user_role_user
        FOREIGN KEY (user_id) REFERENCES sys_user(id) ON DELETE CASCADE,
    CONSTRAINT fk_sys_user_role_role
        FOREIGN KEY (role_id) REFERENCES sys_role(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sys_role_permission (
    id BIGINT PRIMARY KEY,
    role_id BIGINT NOT NULL,
    permission_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    CONSTRAINT uk_sys_role_permission_role_permission UNIQUE (role_id, permission_id),
    CONSTRAINT fk_sys_role_permission_role
        FOREIGN KEY (role_id) REFERENCES sys_role(id) ON DELETE CASCADE,
    CONSTRAINT fk_sys_role_permission_permission
        FOREIGN KEY (permission_id) REFERENCES sys_permission(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sys_user_post (
    id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    post_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    CONSTRAINT uk_sys_user_post_user_post UNIQUE (user_id, post_id),
    CONSTRAINT fk_sys_user_post_user
        FOREIGN KEY (user_id) REFERENCES sys_user(id) ON DELETE CASCADE,
    CONSTRAINT fk_sys_user_post_post
        FOREIGN KEY (post_id) REFERENCES sys_post(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sys_dict_type (
    id BIGINT PRIMARY KEY,
    dict_name VARCHAR(64) NOT NULL,
    dict_code VARCHAR(64) NOT NULL UNIQUE,
    status SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    updated_by BIGINT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    remark VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS sys_dict_item (
    id BIGINT PRIMARY KEY,
    dict_type_id BIGINT NOT NULL,
    item_label VARCHAR(64) NOT NULL,
    item_value VARCHAR(64) NOT NULL,
    item_color VARCHAR(32),
    sort_no INTEGER NOT NULL DEFAULT 0,
    status SMALLINT NOT NULL DEFAULT 1,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    updated_by BIGINT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    remark VARCHAR(500),
    CONSTRAINT uk_sys_dict_item_type_value UNIQUE (dict_type_id, item_value),
    CONSTRAINT fk_sys_dict_item_type
        FOREIGN KEY (dict_type_id) REFERENCES sys_dict_type(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sys_config (
    id BIGINT PRIMARY KEY,
    config_name VARCHAR(128) NOT NULL,
    config_key VARCHAR(128) NOT NULL UNIQUE,
    config_value TEXT,
    value_type VARCHAR(32),
    is_builtin BOOLEAN NOT NULL DEFAULT FALSE,
    status SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by BIGINT,
    updated_by BIGINT,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    remark VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS sys_login_log (
    id BIGINT PRIMARY KEY,
    username VARCHAR(64),
    login_status SMALLINT NOT NULL,
    login_message VARCHAR(255),
    login_ip VARCHAR(64),
    login_location VARCHAR(128),
    user_agent VARCHAR(500),
    browser VARCHAR(64),
    os VARCHAR(64),
    login_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sys_operation_log (
    id BIGINT PRIMARY KEY,
    module_name VARCHAR(64),
    business_type VARCHAR(32),
    permission_code VARCHAR(128),
    request_method VARCHAR(16),
    request_path VARCHAR(255),
    operator_user_id BIGINT,
    operator_name VARCHAR(64),
    operation_ip VARCHAR(64),
    operation_location VARCHAR(128),
    request_params JSONB,
    response_body JSONB,
    operation_status SMALLINT NOT NULL,
    error_message TEXT,
    duration_ms INTEGER,
    operated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE sys_dept
    ADD CONSTRAINT fk_sys_dept_parent
    FOREIGN KEY (parent_id) REFERENCES sys_dept(id) DEFERRABLE INITIALLY DEFERRED;

ALTER TABLE sys_dept
    ADD CONSTRAINT fk_sys_dept_leader_user
    FOREIGN KEY (leader_user_id) REFERENCES sys_user(id) DEFERRABLE INITIALLY DEFERRED;

CREATE INDEX IF NOT EXISTS idx_sys_user_dept_id ON sys_user(dept_id);
CREATE INDEX IF NOT EXISTS idx_sys_user_status ON sys_user(status);
CREATE INDEX IF NOT EXISTS idx_sys_user_deleted_status ON sys_user(is_deleted, status);

CREATE INDEX IF NOT EXISTS idx_sys_role_status ON sys_role(status);
CREATE INDEX IF NOT EXISTS idx_sys_role_sort ON sys_role(role_sort);
CREATE INDEX IF NOT EXISTS idx_sys_role_deleted_status ON sys_role(is_deleted, status);

CREATE INDEX IF NOT EXISTS idx_sys_permission_type ON sys_permission(permission_type);
CREATE INDEX IF NOT EXISTS idx_sys_permission_api_path ON sys_permission(api_path);
CREATE INDEX IF NOT EXISTS idx_sys_permission_deleted_status ON sys_permission(is_deleted, status);

CREATE INDEX IF NOT EXISTS idx_sys_menu_parent_id ON sys_menu(parent_id);
CREATE INDEX IF NOT EXISTS idx_sys_menu_sort_no ON sys_menu(sort_no);
CREATE INDEX IF NOT EXISTS idx_sys_menu_permission_code ON sys_menu(permission_code);
CREATE INDEX IF NOT EXISTS idx_sys_menu_deleted_status ON sys_menu(is_deleted, status);

CREATE INDEX IF NOT EXISTS idx_sys_user_role_user_id ON sys_user_role(user_id);
CREATE INDEX IF NOT EXISTS idx_sys_user_role_role_id ON sys_user_role(role_id);

CREATE INDEX IF NOT EXISTS idx_sys_role_permission_role_id ON sys_role_permission(role_id);
CREATE INDEX IF NOT EXISTS idx_sys_role_permission_permission_id ON sys_role_permission(permission_id);

CREATE INDEX IF NOT EXISTS idx_sys_dept_parent_id ON sys_dept(parent_id);
CREATE INDEX IF NOT EXISTS idx_sys_dept_status ON sys_dept(status);
CREATE INDEX IF NOT EXISTS idx_sys_dept_deleted_status ON sys_dept(is_deleted, status);

CREATE INDEX IF NOT EXISTS idx_sys_post_status ON sys_post(status);
CREATE INDEX IF NOT EXISTS idx_sys_post_deleted_status ON sys_post(is_deleted, status);

CREATE INDEX IF NOT EXISTS idx_sys_user_post_user_id ON sys_user_post(user_id);
CREATE INDEX IF NOT EXISTS idx_sys_user_post_post_id ON sys_user_post(post_id);

CREATE INDEX IF NOT EXISTS idx_sys_dict_type_deleted_status ON sys_dict_type(is_deleted, status);
CREATE INDEX IF NOT EXISTS idx_sys_dict_item_type_id ON sys_dict_item(dict_type_id);
CREATE INDEX IF NOT EXISTS idx_sys_dict_item_deleted_status ON sys_dict_item(is_deleted, status);

CREATE INDEX IF NOT EXISTS idx_sys_config_deleted_status ON sys_config(is_deleted, status);

CREATE INDEX IF NOT EXISTS idx_sys_login_log_username ON sys_login_log(username);
CREATE INDEX IF NOT EXISTS idx_sys_login_log_login_status ON sys_login_log(login_status);
CREATE INDEX IF NOT EXISTS idx_sys_login_log_login_at ON sys_login_log(login_at);

CREATE INDEX IF NOT EXISTS idx_sys_operation_log_operator_user_id ON sys_operation_log(operator_user_id);
CREATE INDEX IF NOT EXISTS idx_sys_operation_log_permission_code ON sys_operation_log(permission_code);
CREATE INDEX IF NOT EXISTS idx_sys_operation_log_operated_at ON sys_operation_log(operated_at);
CREATE INDEX IF NOT EXISTS idx_sys_operation_log_request_path ON sys_operation_log(request_path);

COMMIT;
