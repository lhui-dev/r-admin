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
