pub mod video_downloader {
    use std::borrow::Cow;

    use rustube::{
        url::Url, 
        VideoFetcher,
        Error as Rerror,
        video_info::player_response::video_details::Thumbnail, 
    };

    pub struct Video {
        video: rustube::Video,
        pub thumbnails: Vec<Thumbnail>,
        pub is_private: bool,
        pub title: String,
        pub url: Url,
    }
    impl Video {
        pub async fn new(url: String) -> Result<Video, Rerror> {
            let url = Url::parse(url.as_str()).unwrap();
            
            let descrambler = VideoFetcher::from_url(&url)?.fetch().await?;

            let video = descrambler.clone().descramble()?;

            Ok(Video {
                thumbnails: descrambler.video_details().thumbnails.clone(),
                is_private: descrambler.video_details().is_private,
                title: descrambler.video_title().clone(),
                url,
                // private
                video
            })
        }
        pub async fn get_best_quality(&self) -> Result<(), Rerror>{
            if self.is_private {
                return Err(
                    Rerror::Custom(Cow::from("video is private"))
                );
            }
            let stream = match self.video.best_quality() {
                Some(stream) => stream,
                None => return Err(
                    Rerror::Custom(Cow::from("no stream found"))
                )
            };

            println!("Downloading {}...", self.title);
            stream.download_to(format!("{}",self.title)).await?;

            return Ok(())
        }
        pub async fn get_worst_quality(&self) -> Result<(), Rerror>{
            if self.is_private {
                return Err(Rerror::Custom(Cow::from("value is private")));
            }

            println!("Downloading {}...", self.title);
            let stream = match self.video.worst_quality() {
                Some(stream) => stream,
                None => {
                    return Err(Rerror::Custom(Cow::from("no stream found")));
                }
            };

            stream.download_to(format!("{}",self.title)).await?;

            return Ok(())
        }
        pub async fn get_audio_stream(&self) -> Result<(), Rerror> {
            if self.is_private {
                return Err(Rerror::Custom(Cow::from("value is private")));
            }

            println!("Downloading {}...", self.title);
            let stream = match self.video.best_audio() {
                Some(stream) => stream,
                None => {
                    return Err(Rerror::Custom(Cow::from("no stream found")));
                }
            };
        
            
            stream.download_to(format!("{}",self.title)).await?;

            return Ok(())
        }
    }
}


#[cfg(test)]
mod tests {
}
