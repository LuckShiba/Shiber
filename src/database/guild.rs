// use mongodb::{Collection, Database, bson::doc};

// pub struct GuildUtils {
//     collection: Collection,
//     guild_id: String
// }

// impl GuildUtils {
//     pub fn new(database: Database, guild_id: String) -> Self {
//         Self {
//             collection: database.collection("guilds"),
//             guild_id
//         }
//     }
//     pub async fn get_prefix(self) -> Result<String, ()>{
//         let guild = self.collection
//             .find_one(doc! {
//                 "_id": self.guild_id
//             }, None)
//             .await?;
//         self.collection.find
//         if (let )
//         return Ok(guild.unwrap().get_str("_id").unwrap().to_string());
        
//         Err(())
//     }
// }