use mongodb::bson::{doc, Document};

fn main() {}

pub struct Object {
    pub id: String,
    pub object_guid: String,
    pub updated_at: i64,
    pub created_at: i64,
    pub domain: String,
    pub cn: String,
    pub display_name: String,
    pub distinguished_name: String,
    pub employee_id: String,
    pub mail: String,
    pub mail_nick_name: String,
    pub mobile: String,
    pub object_sid: String,
    pub sam_account_name: String,
    pub sam_account_type: String,
    pub title: String,
    pub department: String,
    pub division: String,
    pub telephone_number: String,
    pub user_principal_name: String,
    pub archived: bool,
    pub deleted: bool,
    pub member_of: Vec<String>,
    pub managed_objects: Vec<String>,
    pub sn: String,
    pub manager: String,
    pub location: String,
    pub company: String,
    pub street_address: String,
    pub physical_delivery_office_name: String,
    pub user_account_control: i64,
    pub when_created: i64,
    pub when_changed: i64,
    pub account_expires: i64,
    pub bad_password_time: i64,
}

impl Object {
    fn to_doc(self) -> Document {
        let mut member_of = doc! {};
        for g in self.member_of {
            member_of.insert(g, true);
        }

        let mut managed_objects = doc! {};
        for o in self.managed_objects {
            managed_objects.insert(o, true);
        }

        doc! {
            "$set": doc! {
                "updatedAt": self.updated_at,
                "domain": self.domain,
                "cn": self.cn,
                "displayName": self.display_name,
                "distinguishedName": self.distinguished_name,
                "employeeID": self.employee_id,
                "mail": self.mail,
                "mailNickname": self.mail_nick_name,
                "mobile": self.mobile,
                "objectSID": self.object_sid,
                "sAMAccountName": self.sam_account_name,
                "telephoneNumber": self.telephone_number,
                "userPrincipalName": self.user_principal_name,
                "deleted": self.deleted,
                "memberOf": member_of,
                "sn": self.sn,
                "sAMAccountType": self.sam_account_type,
                "title": self.title,
                "division": self.division,
                "department": self.department,
                "manager": self.manager,
                "location": self.location,
                "company": self.company,
                "streetAddress": self.street_address,
                "physicalDeliveryOfficeName": self.physical_delivery_office_name,
                "managedObjects": managed_objects,
                "userAccountControl": self.user_account_control,
                "whenCreated": self.when_created,
                "whenChanged": self.when_changed,
                "accountExpires": self.account_expires,
                "badPasswordTime": self.bad_password_time,
                "archived": self.archived,
            },
            "$setOnInsert": doc! {
                "_id": self.id,
                "objectGUID": self.object_guid,
                "createdAt": self.created_at,
            },
        }
    }
}
