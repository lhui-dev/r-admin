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

const topTitles = json.data.menus.map((item) => item.title);
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
