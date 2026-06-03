# 用户管理接口设计

## 1. 目标

本文档用于承接当前认证与动态菜单阶段完成后的下一步工作，先把“用户管理”这一块的后端接口边界定义清楚，作为后续 backend 实现与前后端联调的统一基线。

当前目标不是一次性做完完整用户中心，而是先明确：

1. 第一版用户管理需要哪些接口
2. 列表、详情、创建、更新、状态变更分别返回什么
3. 哪些字段直接基于当前库表即可落地
4. 哪些能力留给后续增强阶段

一句话目标：

`先把“如何查询和维护系统用户”这条主链路定稳，再逐步叠加更复杂的数据权限和安全策略。`

## 2. 当前基础

当前项目已经具备：

1. 登录与当前用户接口
2. 动态菜单初始化
3. PostgreSQL 用户、角色、部门、岗位相关表
4. Docker + APIFox 联调能力

当前与用户管理直接相关的表：

1. `sys_user`
2. `sys_dept`
3. `sys_post`
4. `sys_role`
5. `sys_user_role`
6. `sys_user_post`

当前这意味着：

1. 用户基础资料已有库表支撑
2. 用户与角色、岗位的关联关系已有库表支撑
3. 第一版用户管理接口完全可以在现有数据库设计上直接落地

## 3. 第一版范围

第一版建议纳入：

1. 用户分页列表
2. 用户详情查询
3. 新建用户
4. 更新用户基础信息
5. 修改用户状态

第一版暂不建议纳入：

1. 重置密码
2. 分配角色
3. 分配岗位
4. 批量导入导出
5. 数据权限隔离
6. 用户删除恢复
7. 个人资料自助修改

一句话原则：

`先把后台最常用的“查、增、改、启停”做出来，再继续扩展角色分配和安全操作。`

## 4. 推荐接口清单

第一版建议优先定义以下接口：

1. `GET /api/system/users`
2. `GET /api/system/users/{id}`
3. `POST /api/system/users`
4. `PATCH /api/system/users/{id}`
5. `PATCH /api/system/users/{id}/status`

后续第二批可以接：

1. `PATCH /api/system/users/{id}/password`
2. `PUT /api/system/users/{id}/roles`
3. `PUT /api/system/users/{id}/posts`
4. `DELETE /api/system/users/{id}`

## 5. 列表接口设计

### 5.1 路径

```text
GET /api/system/users
```

### 5.2 查询参数建议

建议第一版支持：

1. `page`
2. `page_size`
3. `keyword`
4. `dept_id`
5. `status`

说明：

1. `keyword` 同时匹配 `username / nickname / real_name / mobile`
2. `dept_id` 用于部门筛选
3. `status` 用于启用/停用筛选

示例：

```text
GET /api/system/users?page=1&page_size=20&keyword=admin&status=1
```

### 5.3 返回结构建议

建议统一采用分页结构：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "items": [
      {
        "id": 1000,
        "username": "admin",
        "nickname": "超级管理员",
        "real_name": "平台管理员",
        "mobile": "13800000000",
        "email": "admin@example.com",
        "status": 1,
        "is_super_admin": true,
        "dept": {
          "id": 100,
          "name": "平台总部"
        },
        "roles": [
          {
            "id": 300,
            "code": "super_admin",
            "name": "超级管理员"
          }
        ],
        "posts": [
          {
            "id": 200,
            "code": "SUPER_ADMIN",
            "name": "超级管理员"
          }
        ],
        "last_login_at": "2026-06-01T12:00:00+08:00",
        "created_at": "2026-05-28T10:00:00+08:00"
      }
    ],
    "pagination": {
      "page": 1,
      "page_size": 20,
      "total": 1
    }
  }
}
```

### 5.4 为什么列表就返回角色与岗位摘要

建议第一版列表就返回轻量角色和岗位摘要，原因：

1. 用户管理页通常需要直接展示“所属角色”
2. 后续不用为列表页再补额外二次请求
3. 第一版数据量不大，复杂度可控

## 6. 详情接口设计

### 6.1 路径

```text
GET /api/system/users/{id}
```

### 6.2 返回结构建议

建议比列表更完整一些：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "id": 1010,
    "username": "sysadmin",
    "nickname": "系统管理员",
    "real_name": "系统管理员账号",
    "mobile": "13800000001",
    "email": "sysadmin@example.com",
    "avatar_url": null,
    "gender": 1,
    "status": 1,
    "is_super_admin": false,
    "remark": "系统初始化系统管理员账号",
    "dept": {
      "id": 100,
      "name": "平台总部"
    },
    "roles": [
      {
        "id": 310,
        "code": "system_admin",
        "name": "系统管理员"
      }
    ],
    "posts": [],
    "last_login_at": "2026-06-01T12:00:00+08:00",
    "last_login_ip": "127.0.0.1",
    "password_updated_at": "2026-05-28T10:00:00+08:00",
    "created_at": "2026-05-28T10:00:00+08:00",
    "updated_at": "2026-06-01T12:00:00+08:00"
  }
}
```

## 7. 创建接口设计

### 7.1 路径

```text
POST /api/system/users
```

### 7.2 请求体建议

第一版建议先支持：

```json
{
  "username": "operator01",
  "password": "Admin@123456",
  "nickname": "操作员01",
  "real_name": "张三",
  "mobile": "13800000003",
  "email": "operator01@example.com",
  "gender": 1,
  "dept_id": 100,
  "status": 1,
  "remark": "首批操作员"
}
```

### 7.3 第一版字段建议

建议第一版只允许写入：

1. `username`
2. `password`
3. `nickname`
4. `real_name`
5. `mobile`
6. `email`
7. `gender`
8. `dept_id`
9. `status`
10. `remark`

第一版建议暂不在创建接口中直接分配角色和岗位。

### 7.4 返回建议

建议成功后直接返回新建用户详情摘要：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "id": 1030,
    "username": "operator01"
  }
}
```

## 8. 更新接口设计

### 8.1 路径

```text
PATCH /api/system/users/{id}
```

### 8.2 请求体建议

```json
{
  "nickname": "操作员01",
  "real_name": "张三",
  "mobile": "13800000003",
  "email": "operator01@example.com",
  "gender": 1,
  "dept_id": 100,
  "remark": "更新说明"
}
```

### 8.3 更新范围建议

第一版建议可更新：

1. `nickname`
2. `real_name`
3. `mobile`
4. `email`
5. `gender`
6. `dept_id`
7. `remark`

第一版建议不要通过这个接口更新：

1. `username`
2. `password`
3. `is_super_admin`
4. `status`

这些建议拆到独立接口或独立流程，避免职责混乱。

## 9. 状态变更接口设计

### 9.1 路径

```text
PATCH /api/system/users/{id}/status
```

### 9.2 请求体建议

```json
{
  "status": 0
}
```

### 9.3 规则建议

建议：

1. 只允许 `0 / 1`
2. 超级管理员账号不允许被随意停用
3. 当前登录用户不允许停用自己

这些规则即便第一版不全做，也建议文档先明确。

## 10. DTO 设计建议

建议在 backend 中预留：

```text
backend/src/modules/system_user/
├─ dto.rs
├─ handler.rs
├─ service.rs
└─ mod.rs
```

建议第一版 DTO 至少包括：

1. `UserListQuery`
2. `UserListItem`
3. `UserListResponse`
4. `UserDetailResponse`
5. `CreateUserRequest`
6. `CreateUserResponse`
7. `UpdateUserRequest`
8. `UpdateUserStatusRequest`

## 11. 查询实现建议

列表查询建议：

1. 主表从 `sys_user` 查
2. 左连接 `sys_dept`
3. 角色和岗位可先二次查询聚合
4. 第一版优先保证结果清晰，不必过度追求单 SQL 极限优化

详情查询建议：

1. `sys_user` 查询主记录
2. `sys_user_role -> sys_role` 查询角色
3. `sys_user_post -> sys_post` 查询岗位
4. `sys_dept` 查询部门

## 12. 错误语义建议

建议保持与当前项目统一风格：

1. 参数错误：`400`
2. 未登录或 token 无效：`401`
3. 无权限：`403`
4. 用户不存在：`404`
5. 用户名/手机号/邮箱冲突：`409`

## 13. 第一版完成标准

当下面条件满足时，可认为用户管理接口第一版已经具备进入真实编码的条件：

1. 列表接口边界明确
2. 详情接口边界明确
3. 创建与更新字段边界明确
4. 状态变更接口边界明确
5. DTO 与模块落位明确

## 14. 推荐下一步

建议后续按下面顺序推进：

1. 将本文档纳入 docs 索引
2. 在 backend 中新增 `system_user` 模块骨架
3. 先落 DTO 与 handler 占位
4. 再实现列表与详情
5. 最后补创建、更新、状态变更

一句话总结：

`用户管理是 RBAC 业务接口的第一站，先把接口契约定稳，后续角色管理和菜单管理就能沿用同一套风格展开。`
