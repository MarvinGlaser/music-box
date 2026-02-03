use zbus::{Connection, fdo::DBusProxy, names::OwnedBusName};

pub struct DbusScanner<'a> {
    dbus_proxy: DBusProxy<'a>
}

impl<'a> DbusScanner<'a> {

    pub async fn new(connection: &Connection) -> zbus::Result<Self> {
        let proxy = DBusProxy::new(connection).await?;
        Ok(Self { dbus_proxy: proxy })
    }

    pub async fn get_server_vec(self, name_start: &str) -> zbus::Result<Vec<OwnedBusName>> {
        let names = self.dbus_proxy.list_names().await?;
        let scan_result = names
            .into_iter()
            .filter(|n| n.starts_with(name_start))
            .collect();
        Ok(scan_result)

    }

}

