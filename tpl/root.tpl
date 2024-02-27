<html>
<head>
  <title>dnsmasq</title>
  <link rel="stylesheet" href="dist/output.css">
</head>
<body>


<div class="overflow-x-auto">
  <table class="table table-zebra">
    <!-- head -->
    <thead>
      <tr>
        <th>time</th>
        <th>host</th>
        <th>link-name</th>
        <th>link-ip</th>
        <th>mac</th>
        <th>ip</th>
      </tr>
    </thead>
    <tbody>
      <!-- row 1 -->
      {% for entry in entries %}
      <tr>
      <td>{{entry.last_change}}</td>
      <td>{{entry.name}}</td>
      <td><a href="http://{{ entry.name }}" target="_blank">http://{{ entry.name }}</a></td>
      <td><a href="http://{{ entry.ip }}" target="_blank">http://{{ entry.ip }}</td>
      <td>{{entry.mac}}</td>
      <td>{{entry.ip}}</td>
      </tr>
      {% endfor %}
    </tbody>
  </table>
</div>

</body>
</html>
