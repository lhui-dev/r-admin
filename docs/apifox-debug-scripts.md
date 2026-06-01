# APIFox 调试脚本清单

本文档整理了一组适用于当前 `r-admin` 认证与菜单接口的 APIFox 脚本，目标是：

1. 自动保存登录返回的 token
2. 自动拼接 `Authorization` 请求头
3. 自动校验 `/me` 与 `/menus` 返回结构
4. 自动清理退出登录后的本地变量

建议搭配文档：

1. [APIFox 认证调试说明](./apifox-auth-debug.md)
2. [认证接口 OpenAPI 导入文件](./apifox-auth.openapi.json)

当前仓库已附带可直接复制的脚本文件：

1. [通用鉴权前置脚本](./apifox-scripts/auth-prerequest.js)
2. [登录前置脚本](./apifox-scripts/login-prerequest.js)
3. [登录后置脚本](./apifox-scripts/login-postresponse.js)
4. [`/me` 后置脚本](./apifox-scripts/me-postresponse.js)
5. [`/menus` 后置脚本](./apifox-scripts/menus-postresponse.js)
6. [退出后置脚本](./apifox-scripts/logout-postresponse.js)

脚本约定：

1. 当前统一采用 APIFox 兼容的 `pm.*` 写法
2. 这些脚本适合放在“前置脚本”或“后置脚本/测试脚本”位置
3. 当前未使用请求编排类能力，因此不依赖 `pm.nextRequest`

## 1. 推荐环境变量

建议在 APIFox 本地环境中预先创建这些变量：

```text
baseUrl = http://127.0.0.1:8080
access_token =
auth_header =
current_user_id =
current_username =
current_roles =
current_permissions =
current_menu_titles =
login_username = admin
login_password = Admin@123456
expected_role =
```

说明：

1. `auth_header` 建议由前置脚本自动生成，不手工维护
2. `current_roles`、`current_permissions`、`current_menu_titles` 建议存成 JSON 字符串
3. `expected_role` 用于不同账号调试时做断言，例如 `super_admin`、`system_admin`、`auditor`

## 2. 鉴权接口通用前置脚本

适用范围：

1. `GET /api/auth/me`
2. `GET /api/auth/menus`
3. `POST /api/auth/logout`

建议加在这些需要登录的接口的前置脚本中：

```javascript
const token = pm.environment.get('access_token');

if (!token) {
  throw new Error('缺少 access_token，请先执行登录接口。');
}

const authHeader = `Bearer ${token}`;

pm.environment.set('auth_header', authHeader);
pm.request.headers.upsert({
  key: 'Authorization',
  value: authHeader,
});
```

你可以二选一：

1. 直接依赖脚本动态写入请求头
2. 或者在接口请求头中保留一个显式配置

如果你喜欢显式配置，请写：

```text
Authorization: {{auth_header}}
```

## 3. 登录接口脚本

### 3.1 登录前置脚本

适用接口：

1. `POST /api/auth/login`

作用：

1. 自动从环境变量读取登录账号
2. 自动生成请求体变量
3. 避免直接改写请求体对象时的兼容性问题

如果你希望在 APIFox 中切换账号时少改请求体，可以使用下面脚本：

```javascript
const username = pm.environment.get('login_username') || 'admin';
const password = pm.environment.get('login_password') || 'Admin@123456';

pm.variables.set('request_login_username', username);
pm.variables.set('request_login_password', password);
pm.variables.set(
  'request_login_body',
  JSON.stringify({
    username,
    password,
  })
);
```

然后把登录接口请求体改成下面两种方式之一。

方式 A：整段 raw body 使用一个变量

```json
{{request_login_body}}
```

方式 B：保留 JSON 结构，用字段变量

```json
{
  "username": "{{request_login_username}}",
  "password": "{{request_login_password}}"
}
```

### 3.2 登录后置脚本

作用：

1. 断言登录成功
2. 自动保存 token 和用户信息
3. 生成后续接口复用的环境变量

```javascript
pm.test('login status is 200', function () {
  pm.response.to.have.status(200);
});

const json = pm.response.json();

pm.test('login business code is 0', function () {
  pm.expect(json.code).to.eql(0);
});

pm.test('login payload contains token and user', function () {
  pm.expect(json.data).to.have.property('access_token');
  pm.expect(json.data).to.have.property('user');
  pm.expect(json.data.user).to.have.property('username');
});

pm.environment.set('access_token', json.data.access_token);
pm.environment.set('auth_header', `Bearer ${json.data.access_token}`);
pm.environment.set('current_user_id', String(json.data.user.id));
pm.environment.set('current_username', json.data.user.username);
pm.environment.set('current_roles', JSON.stringify([]));
pm.environment.set('current_permissions', JSON.stringify([]));

console.log('login user:', json.data.user.username);
console.log('token saved to environment');
```

## 4. `/api/auth/me` 后置脚本

适用接口：

1. `GET /api/auth/me`

作用：

1. 校验用户信息、角色、权限
2. 保存角色和权限，给后续菜单断言复用

```javascript
pm.test('me status is 200', function () {
  pm.response.to.have.status(200);
});

const json = pm.response.json();

pm.test('me business code is 0', function () {
  pm.expect(json.code).to.eql(0);
});

pm.test('me payload shape is valid', function () {
  pm.expect(json.data.user).to.be.an('object');
  pm.expect(json.data.roles).to.be.an('array');
  pm.expect(json.data.permissions).to.be.an('array');
});

pm.environment.set('current_user_id', String(json.data.user.id));
pm.environment.set('current_username', json.data.user.username);
pm.environment.set('current_roles', JSON.stringify(json.data.roles));
pm.environment.set('current_permissions', JSON.stringify(json.data.permissions));

const expectedRole = pm.environment.get('expected_role');
if (expectedRole) {
  pm.test(`me contains expected role: ${expectedRole}`, function () {
    pm.expect(json.data.roles).to.include(expectedRole);
  });
}

console.log('roles:', json.data.roles);
console.log('permissions count:', json.data.permissions.length);
```

## 5. `/api/auth/menus` 后置脚本

适用接口：

1. `GET /api/auth/menus`

作用：

1. 校验菜单树结构
2. 保存一级菜单标题
3. 根据不同角色做菜单裁剪断言

```javascript
pm.test('menus status is 200', function () {
  pm.response.to.have.status(200);
});

const json = pm.response.json();

pm.test('menus business code is 0', function () {
  pm.expect(json.code).to.eql(0);
});

pm.test('menus payload shape is valid', function () {
  pm.expect(json.data).to.have.property('menus');
  pm.expect(json.data.menus).to.be.an('array');
});

const topTitles = json.data.menus.map(item => item.title);
pm.environment.set('current_menu_titles', JSON.stringify(topTitles));

json.data.menus.forEach((item) => {
  pm.test(`menu node "${item.title}" has children array`, function () {
    pm.expect(item.children).to.be.an('array');
  });
});

const currentRoles = JSON.parse(pm.environment.get('current_roles') || '[]');

if (currentRoles.includes('super_admin')) {
  pm.test('super admin menus contain system management', function () {
    pm.expect(topTitles).to.include('系统管理');
  });
}

if (currentRoles.includes('auditor')) {
  pm.test('auditor menus do not contain system management', function () {
    pm.expect(topTitles).to.not.include('系统管理');
  });

  pm.test('auditor menus contain audit section', function () {
    pm.expect(topTitles).to.include('日志审计');
  });
}

console.log('top menu titles:', topTitles);
```

## 6. `/api/auth/logout` 后置脚本

适用接口：

1. `POST /api/auth/logout`

作用：

1. 断言退出成功
2. 清理登录态相关环境变量

```javascript
pm.test('logout status is 200', function () {
  pm.response.to.have.status(200);
});

const json = pm.response.json();

pm.test('logout business code is 0', function () {
  pm.expect(json.code).to.eql(0);
});

pm.environment.unset('access_token');
pm.environment.unset('auth_header');
pm.environment.unset('current_user_id');
pm.environment.unset('current_username');
pm.environment.unset('current_roles');
pm.environment.unset('current_permissions');
pm.environment.unset('current_menu_titles');

console.log('auth environment cleared');
```

## 7. 角色切换建议

为了方便你快速切换调试账号，建议只改这 3 个环境变量：

```text
login_username = admin | sysadmin | auditor
login_password = Admin@123456
expected_role = super_admin | system_admin | auditor
```

推荐组合：

1. `admin + super_admin`
2. `sysadmin + system_admin`
3. `auditor + auditor`

## 8. 推荐调试流程

建议按这个顺序执行：

1. 执行 `POST /api/auth/login`
2. 确认环境里已写入 `access_token`
3. 执行 `GET /api/auth/me`
4. 执行 `GET /api/auth/menus`
5. 如需结束当前会话，再执行 `POST /api/auth/logout`

## 9. 接口级绑定建议

下面这张表可以直接指导你在 APIFox 里给每个接口贴脚本。

| 接口 | 前置脚本 | 后置脚本 | 额外配置 |
| --- | --- | --- | --- |
| `GET /api/health` | 无 | 可选，仅做状态断言 | 无 |
| `POST /api/auth/login` | [login-prerequest.js](./apifox-scripts/login-prerequest.js) | [login-postresponse.js](./apifox-scripts/login-postresponse.js) | 请求体建议使用 `{{request_login_body}}` 或字段变量 |
| `GET /api/auth/me` | [auth-prerequest.js](./apifox-scripts/auth-prerequest.js) | [me-postresponse.js](./apifox-scripts/me-postresponse.js) | 可不手工写请求头，也可保留 `Authorization: {{auth_header}}` |
| `GET /api/auth/menus` | [auth-prerequest.js](./apifox-scripts/auth-prerequest.js) | [menus-postresponse.js](./apifox-scripts/menus-postresponse.js) | 可不手工写请求头，也可保留 `Authorization: {{auth_header}}` |
| `POST /api/auth/logout` | [auth-prerequest.js](./apifox-scripts/auth-prerequest.js) | [logout-postresponse.js](./apifox-scripts/logout-postresponse.js) | 可不手工写请求头，也可保留 `Authorization: {{auth_header}}` |

### 9.1 `GET /api/health`

这个接口可以不挂脚本。

如果你希望也做一个最小断言，可以在后置脚本里放：

```javascript
pm.test('health status is 200', function () {
  pm.response.to.have.status(200);
});

const json = pm.response.json();

pm.test('health business code is 0', function () {
  pm.expect(json.code).to.eql(0);
});

pm.test('database is up', function () {
  pm.expect(json.data.database).to.eql('up');
});
```

### 9.2 `POST /api/auth/login`

建议绑定：

1. 前置脚本：`login-prerequest.js`
2. 后置脚本：`login-postresponse.js`

建议请求体保留为 JSON 模式，并使用变量占位。

推荐请求体：

```json
{
  "username": "{{request_login_username}}",
  "password": "{{request_login_password}}"
}
```

推荐请求头：

```text
Content-Type: application/json
```

### 9.3 `GET /api/auth/me`

建议绑定：

1. 前置脚本：`auth-prerequest.js`
2. 后置脚本：`me-postresponse.js`

如果你希望显式保留请求头配置，推荐写：

```text
Authorization: {{auth_header}}
```

### 9.4 `GET /api/auth/menus`

建议绑定：

1. 前置脚本：`auth-prerequest.js`
2. 后置脚本：`menus-postresponse.js`

如果你希望显式保留请求头配置，推荐写：

```text
Authorization: {{auth_header}}
```

### 9.5 `POST /api/auth/logout`

建议绑定：

1. 前置脚本：`auth-prerequest.js`
2. 后置脚本：`logout-postresponse.js`

如果你希望显式保留请求头配置，推荐写：

```text
Authorization: {{auth_header}}
```

### 9.6 建议的 APIFox 分组级复用方式

如果你后面接口会越来越多，推荐这样挂：

1. 在 `Auth` 分组下的“需要登录接口”统一挂 `auth-prerequest.js`
2. `login` 单独覆盖自己的前置脚本
3. 每个接口再单独挂自己的后置脚本

这样后续新增接口时，只需要：

1. 继承分组级鉴权前置脚本
2. 补一个本接口最小断言脚本

## 10. 后续扩展建议

后面如果新增接口，建议复用这套思路：

1. 登录接口负责写 token
2. 鉴权接口统一读取 `{{auth_header}}`
3. 用户态接口把关键字段写回环境变量
4. 列表接口尽量补上最小结构断言
5. 角色差异接口尽量补上“正向 + 反向”断言
