use crate::errors::{AppErrorType::*, AppError};
use crate::models::{Article, Rest, Ciamis, CountryCovid};

use select::document::Document;
use select::predicate::{Attr, Class, Name};

use slog::{crit, o, Logger};

pub async fn get_article_asy_syariah(log: Logger) -> Result<Rest, AppError> {
    let body = reqwest::get("https://asysyariah.com/?s=corona").await?.text().await.map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error request");
        AppError::from(err)
    });

    match body {
        Ok(rs) => {
            let document = Document::from(rs.as_str());
            let mut rest_articles = Vec::new();
            for node in document.find(Class("jeg_post_title")) {
                let node_a = node.find(Name("a")).next().unwrap();
                let judul = node_a.text();
                let link = node_a.attr("href").unwrap();
                rest_articles.push(Article{judul: judul, url: link.to_string()});
            }
            Ok(Rest{
                code: 200,
                status: "Ok".to_string(),
                data: Some(rest_articles),
            })
        },
        Err(e) => Err(AppError::from(e)),
    }
}

pub async fn get_article_nasehat(log: Logger) -> Result<Rest, AppError> {
    let body = reqwest::get("https://www.tanggapcovid19.com/category/nasehat/")
    .await?
    .text()
    .await
    .map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error request");
        AppError::from(err)
    });

    match body {
        Ok(rs) => {
            let document = Document::from(rs.as_str());
            let mut vc = Vec::new();
            for node in document.find(Attr("id", "gmr-main-load"))
            .next()
            .unwrap()
            .parent()
            .unwrap()
            .find(Class("entry-title")) {
                let node_a = node.find(Name("a")).next().unwrap();
                let judul = node_a.text();
                let link = node_a.attr("href").unwrap();

                vc.push(Article{
                    judul: judul,
                    url: link.to_string(),
                });
            }

            Ok(Rest{
                code: 200,
                status: "Ok".to_string(),
                data: Some(vc)
            })
        },
        Err(e) => Err(AppError::from(e)),
    }
}

pub async fn get_article_akhbar(log: Logger) -> Result<Rest, AppError> {
    let body = reqwest::get("https://www.tanggapcovid19.com/category/akhbar/")
    .await?
    .text()
    .await
    .map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error request");
        AppError::from(err)
    });

    match body {
        Ok(rs) => {
            let document = Document::from(rs.as_str());
            let mut vc = Vec::new();
            for node in document.find(Attr("id", "gmr-main-load"))
            .next()
            .unwrap()
            .parent()
            .unwrap()
            .find(Class("entry-title")) {
                let node_a = node.find(Name("a")).next().unwrap();
                let judul = node_a.text();
                let link = node_a.attr("href").unwrap();

                vc.push(Article{
                    judul: judul,
                    url: link.to_string(),
                });
            }

            Ok(Rest{
                code: 200,
                status: "Ok".to_string(),
                data: Some(vc)
            })
        },
        Err(e) => Err(AppError::from(e)),
    }
}

pub async fn get_article_bnpb(log: Logger) -> Result<Rest, AppError> {
    let body = reqwest::get("https://bnpb.go.id/cari?q=covid")
    .await?
    .text()
    .await
    .map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error request");
        AppError::from(err)
    });

    match body {
        Ok(str) => {
            let document = Document::from(str.as_str());
            let mut result = Vec::new();
            for node in document.find(Attr("id", "main"))
            .next()
            .unwrap()
            .parent()
            .unwrap()
            .find(Class("title")) {
                let node_a = node.find(Name("a")).next().unwrap();
                let judul = node_a.text();
                let link = node_a.attr("href").unwrap();
                result.push(Article{
                    judul: judul,
                    url: link.to_string(),
                })
            }

            Ok(Rest{
                code: 200,
                status: "Ok".to_string(),
                data: Some(result)
            })
        },
        Err(e) => Err(AppError::from(e)),
    }
}

pub async fn get_data_ciamis(log: Logger) -> Result<Ciamis, AppError> {
    let body = reqwest::get("https://pikcovid19.ciamiskab.go.id/")
    .await?
    .text()
    .await
    .map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error request");
        AppError::from(err)
    });

    match body {
        Ok(str) => {
            let document = Document::from(str.as_str());
            let node = document.find(Class("fl-row-content-wrap")).collect::<Vec<_>>();
            let node_at = node[4];
            let spans = node_at.find(Name("span"))
            .map(|spn| spn.text())
            .collect::<Vec<_>>();

            Ok(Ciamis{
                code: 200,
                status: "Ok".to_string(),
                data: Some(spans)
            })
        },
        Err(e) => Err(AppError::from(e))
    }
}

pub async fn get_data_covid_per_country(log: Logger) -> Result<Vec<CountryCovid>, AppError> {
    let body = reqwest::get("https://coronavirus.jhu.edu/map.html")
    .await?
    .text()
    .await
    .map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error request");
        AppError::from(err)
    });

    match body {
        Ok(str) => {
            let document = Document::from(str.as_str());
            let mut result = Vec::new();
            for node in document.find(Class("feature-list"))
            .next()
            .unwrap()
            .parent()
            .unwrap()
            .find(Class("external-html")) {
                let tot = node.find(Name("strong")).next().unwrap().text();
                //let neg = node.find(Attr("style", "color:#d6d6d6")).next().unwrap().text();
                result.push(CountryCovid{
                    total: tot,
                    name: ""
                })
            }

            Ok(result)
        },
        Err(e) => Err(AppError::from(e))
    }
}