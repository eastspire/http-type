#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileExtension {
    FileExtension123,
    FileExtension3dml,
    FileExtension3ds,
    FileExtension3g2,
    FileExtension3gp,
    FileExtension7z,
    FileExtensionAab,
    FileExtensionAac,
    FileExtensionAam,
    FileExtensionAas,
    FileExtensionAbs,
    FileExtensionAbw,
    FileExtensionAc,
    FileExtensionAcc,
    FileExtensionAce,
    FileExtensionAcu,
    FileExtensionAcutc,
    FileExtensionAdp,
    FileExtensionAep,
    FileExtensionAfm,
    FileExtensionAfp,
    FileExtensionAhead,
    FileExtensionAi,
    FileExtensionAif,
    FileExtensionAifc,
    FileExtensionAiff,
    FileExtensionAim,
    FileExtensionAir,
    FileExtensionAit,
    FileExtensionAmi,
    FileExtensionAnx,
    FileExtensionApk,
    FileExtensionAppcache,
    FileExtensionApplication,
    FileExtensionApr,
    FileExtensionArc,
    FileExtensionArt,
    FileExtensionAsc,
    FileExtensionAsf,
    FileExtensionAsm,
    FileExtensionAso,
    FileExtensionAsx,
    FileExtensionAtc,
    FileExtensionAtom,
    FileExtensionAtomcat,
    FileExtensionAtomsvc,
    FileExtensionAtx,
    FileExtensionAu,
    FileExtensionAvi,
    FileExtensionAvx,
    FileExtensionAw,
    FileExtensionAxa,
    FileExtensionAxv,
    FileExtensionAzf,
    FileExtensionAzs,
    FileExtensionAzw,
    FileExtensionBat,
    FileExtensionBcpio,
    FileExtensionBdf,
    FileExtensionBdm,
    FileExtensionBed,
    FileExtensionBh2,
    FileExtensionBin,
    FileExtensionBlb,
    FileExtensionBlorb,
    FileExtensionBmi,
    FileExtensionBmp,
    FileExtensionBody,
    FileExtensionBook,
    FileExtensionBox,
    FileExtensionBoz,
    FileExtensionBpk,
    FileExtensionBtif,
    FileExtensionBz,
    FileExtensionBz2,
    FileExtensionC,
    FileExtensionC11amc,
    FileExtensionC11amz,
    FileExtensionC4d,
    FileExtensionC4f,
    FileExtensionC4g,
    FileExtensionC4p,
    FileExtensionC4u,
    FileExtensionCab,
    FileExtensionCaf,
    FileExtensionCap,
    FileExtensionCar,
    FileExtensionCat,
    FileExtensionCb7,
    FileExtensionCba,
    FileExtensionCbr,
    FileExtensionCbt,
    FileExtensionCbz,
    FileExtensionCc,
    FileExtensionCct,
    FileExtensionCcxml,
    FileExtensionCdbcmsg,
    FileExtensionCdf,
    FileExtensionCdkey,
    FileExtensionCdmia,
    FileExtensionCdmic,
    FileExtensionCdmid,
    FileExtensionCdmio,
    FileExtensionCdmiq,
    FileExtensionCdx,
    FileExtensionCdxml,
    FileExtensionCdy,
    FileExtensionCer,
    FileExtensionCfs,
    FileExtensionCgm,
    FileExtensionChat,
    FileExtensionChm,
    FileExtensionChrt,
    FileExtensionCif,
    FileExtensionCii,
    FileExtensionCil,
    FileExtensionCla,
    FileExtensionClass,
    FileExtensionClkk,
    FileExtensionClkp,
    FileExtensionClkt,
    FileExtensionClkw,
    FileExtensionClkx,
    FileExtensionClp,
    FileExtensionCmc,
    FileExtensionCmdf,
    FileExtensionCml,
    FileExtensionCmp,
    FileExtensionCmx,
    FileExtensionCod,
    FileExtensionCom,
    FileExtensionConf,
    FileExtensionCpio,
    FileExtensionCpp,
    FileExtensionCpt,
    FileExtensionCrd,
    FileExtensionCrl,
    FileExtensionCrt,
    FileExtensionCryptonote,
    FileExtensionCsh,
    FileExtensionCsml,
    FileExtensionCsp,
    FileExtensionCss,
    FileExtensionCst,
    FileExtensionCsv,
    FileExtensionCu,
    FileExtensionCurl,
    FileExtensionCww,
    FileExtensionCxt,
    FileExtensionCxx,
    FileExtensionDae,
    FileExtensionDaf,
    FileExtensionDart,
    FileExtensionDataless,
    FileExtensionDavmount,
    FileExtensionDbk,
    FileExtensionDcr,
    FileExtensionDcurl,
    FileExtensionDd2,
    FileExtensionDdd,
    FileExtensionDeb,
    FileExtensionDef,
    FileExtensionDeploy,
    FileExtensionDer,
    FileExtensionDfac,
    FileExtensionDgc,
    FileExtensionDib,
    FileExtensionDic,
    FileExtensionDir,
    FileExtensionDis,
    FileExtensionDist,
    FileExtensionDistz,
    FileExtensionDjv,
    FileExtensionDjvu,
    FileExtensionDll,
    FileExtensionDmg,
    FileExtensionDmp,
    FileExtensionDms,
    FileExtensionDna,
    FileExtensionDoc,
    FileExtensionDocm,
    FileExtensionDocx,
    FileExtensionDot,
    FileExtensionDotm,
    FileExtensionDotx,
    FileExtensionDp,
    FileExtensionDpg,
    FileExtensionDra,
    FileExtensionDsc,
    FileExtensionDssc,
    FileExtensionDtb,
    FileExtensionDtd,
    FileExtensionDts,
    FileExtensionDtshd,
    FileExtensionDump,
    FileExtensionDv,
    FileExtensionDvb,
    FileExtensionDvi,
    FileExtensionDwf,
    FileExtensionDwg,
    FileExtensionDxf,
    FileExtensionDxp,
    FileExtensionDxr,
    FileExtensionEcelp4800,
    FileExtensionEcelp7470,
    FileExtensionEcelp9600,
    FileExtensionEcma,
    FileExtensionEdm,
    FileExtensionEdx,
    FileExtensionEfif,
    FileExtensionEi6,
    FileExtensionElc,
    FileExtensionEmf,
    FileExtensionEml,
    FileExtensionEmma,
    FileExtensionEmz,
    FileExtensionEol,
    FileExtensionEot,
    FileExtensionEps,
    FileExtensionEpub,
    FileExtensionEs3,
    FileExtensionEsa,
    FileExtensionEsf,
    FileExtensionEt3,
    FileExtensionEtx,
    FileExtensionEva,
    FileExtensionEvy,
    FileExtensionExe,
    FileExtensionExi,
    FileExtensionExt,
    FileExtensionEz,
    FileExtensionEz2,
    FileExtensionEz3,
    FileExtensionF,
    FileExtensionF4v,
    FileExtensionF77,
    FileExtensionF90,
    FileExtensionFbs,
    FileExtensionFcdt,
    FileExtensionFcs,
    FileExtensionFdf,
    FileExtensionFeLaunch,
    FileExtensionFg5,
    FileExtensionFgd,
    FileExtensionFh,
    FileExtensionFh4,
    FileExtensionFh5,
    FileExtensionFh7,
    FileExtensionFhc,
    FileExtensionFig,
    FileExtensionFlac,
    FileExtensionFli,
    FileExtensionFlo,
    FileExtensionFlv,
    FileExtensionFlw,
    FileExtensionFlx,
    FileExtensionFly,
    FileExtensionFm,
    FileExtensionFnc,
    FileExtensionFor,
    FileExtensionFpx,
    FileExtensionFrame,
    FileExtensionFsc,
    FileExtensionFst,
    FileExtensionFtc,
    FileExtensionFti,
    FileExtensionFvt,
    FileExtensionFxp,
    FileExtensionFxpl,
    FileExtensionFzs,
    FileExtensionG2w,
    FileExtensionG3,
    FileExtensionG3w,
    FileExtensionGac,
    FileExtensionGam,
    FileExtensionGbr,
    FileExtensionGca,
    FileExtensionGdl,
    FileExtensionGeo,
    FileExtensionGex,
    FileExtensionGgb,
    FileExtensionGgt,
    FileExtensionGhf,
    FileExtensionGif,
    FileExtensionGim,
    FileExtensionGml,
    FileExtensionGmx,
    FileExtensionGnumeric,
    FileExtensionGph,
    FileExtensionGpx,
    FileExtensionGqf,
    FileExtensionGqs,
    FileExtensionGram,
    FileExtensionGramps,
    FileExtensionGre,
    FileExtensionGrv,
    FileExtensionGrxml,
    FileExtensionGsf,
    FileExtensionGtar,
    FileExtensionGtm,
    FileExtensionGtw,
    FileExtensionGv,
    FileExtensionGxf,
    FileExtensionGxt,
    FileExtensionGz,
    FileExtensionH,
    FileExtensionH261,
    FileExtensionH263,
    FileExtensionH264,
    FileExtensionHal,
    FileExtensionHbci,
    FileExtensionHdf,
    FileExtensionHh,
    FileExtensionHlp,
    FileExtensionHpgl,
    FileExtensionHpid,
    FileExtensionHps,
    FileExtensionHqx,
    FileExtensionHtc,
    FileExtensionHtke,
    FileExtensionHtm,
    FileExtensionHtml,
    FileExtensionHvd,
    FileExtensionHvp,
    FileExtensionHvs,
    FileExtensionI2g,
    FileExtensionIcc,
    FileExtensionIce,
    FileExtensionIcm,
    FileExtensionIco,
    FileExtensionIcs,
    FileExtensionIef,
    FileExtensionIfb,
    FileExtensionIfm,
    FileExtensionIges,
    FileExtensionIgl,
    FileExtensionIgm,
    FileExtensionIgs,
    FileExtensionIgx,
    FileExtensionIif,
    FileExtensionImp,
    FileExtensionIms,
    FileExtensionIn,
    FileExtensionInk,
    FileExtensionInkml,
    FileExtensionInstall,
    FileExtensionIota,
    FileExtensionIpfix,
    FileExtensionIpk,
    FileExtensionIrm,
    FileExtensionIrp,
    FileExtensionIso,
    FileExtensionItp,
    FileExtensionIvp,
    FileExtensionIvu,
    FileExtensionJad,
    FileExtensionJam,
    FileExtensionJar,
    FileExtensionJava,
    FileExtensionJisp,
    FileExtensionJlt,
    FileExtensionJnlp,
    FileExtensionJoda,
    FileExtensionJpe,
    FileExtensionJpeg,
    FileExtensionJpg,
    FileExtensionJpgm,
    FileExtensionJpgv,
    FileExtensionJpm,
    FileExtensionJs,
    FileExtensionJsf,
    FileExtensionJson,
    FileExtensionJsonml,
    FileExtensionJspf,
    FileExtensionKar,
    FileExtensionKarbon,
    FileExtensionKfo,
    FileExtensionKia,
    FileExtensionKml,
    FileExtensionKmz,
    FileExtensionKne,
    FileExtensionKnp,
    FileExtensionKon,
    FileExtensionKpr,
    FileExtensionKpt,
    FileExtensionKpxx,
    FileExtensionKsp,
    FileExtensionKtr,
    FileExtensionKtx,
    FileExtensionKtz,
    FileExtensionKwd,
    FileExtensionKwt,
    FileExtensionLasxml,
    FileExtensionLatex,
    FileExtensionLbd,
    FileExtensionLbe,
    FileExtensionLes,
    FileExtensionLha,
    FileExtensionLink66,
    FileExtensionList,
    FileExtensionList3820,
    FileExtensionListafp,
    FileExtensionLnk,
    FileExtensionLog,
    FileExtensionLostxml,
    FileExtensionLrf,
    FileExtensionLrm,
    FileExtensionLtf,
    FileExtensionLvp,
    FileExtensionLwp,
    FileExtensionLzh,
    FileExtensionM13,
    FileExtensionM14,
    FileExtensionM1v,
    FileExtensionM21,
    FileExtensionM2a,
    FileExtensionM2v,
    FileExtensionM3a,
    FileExtensionM3u,
    FileExtensionM3u8,
    FileExtensionM4a,
    FileExtensionM4b,
    FileExtensionM4r,
    FileExtensionM4u,
    FileExtensionM4v,
    FileExtensionMa,
    FileExtensionMac,
    FileExtensionMads,
    FileExtensionMag,
    FileExtensionMaker,
    FileExtensionMan,
    FileExtensionMar,
    FileExtensionMathml,
    FileExtensionMb,
    FileExtensionMbk,
    FileExtensionMbox,
    FileExtensionMc1,
    FileExtensionMcd,
    FileExtensionMcurl,
    FileExtensionMdb,
    FileExtensionMdi,
    FileExtensionMe,
    FileExtensionMesh,
    FileExtensionMeta4,
    FileExtensionMetalink,
    FileExtensionMets,
    FileExtensionMfm,
    FileExtensionMft,
    FileExtensionMgp,
    FileExtensionMgz,
    FileExtensionMid,
    FileExtensionMidi,
    FileExtensionMie,
    FileExtensionMif,
    FileExtensionMime,
    FileExtensionMj2,
    FileExtensionMjp2,
    FileExtensionMk3d,
    FileExtensionMka,
    FileExtensionMks,
    FileExtensionMkv,
    FileExtensionMlp,
    FileExtensionMmd,
    FileExtensionMmf,
    FileExtensionMmr,
    FileExtensionMng,
    FileExtensionMny,
    FileExtensionMobi,
    FileExtensionMods,
    FileExtensionMov,
    FileExtensionMovie,
    FileExtensionMp1,
    FileExtensionMp2,
    FileExtensionMp21,
    FileExtensionMp2a,
    FileExtensionMp3,
    FileExtensionMp4,
    FileExtensionMp4a,
    FileExtensionMp4s,
    FileExtensionMp4v,
    FileExtensionMpa,
    FileExtensionMpc,
    FileExtensionMpe,
    FileExtensionMpeg,
    FileExtensionMpega,
    FileExtensionMpg,
    FileExtensionMpg4,
    FileExtensionMpga,
    FileExtensionMpkg,
    FileExtensionMpm,
    FileExtensionMpn,
    FileExtensionMpp,
    FileExtensionMpt,
    FileExtensionMpv2,
    FileExtensionMpy,
    FileExtensionMqy,
    FileExtensionMrc,
    FileExtensionMrcx,
    FileExtensionMs,
    FileExtensionMscml,
    FileExtensionMseed,
    FileExtensionMseq,
    FileExtensionMsf,
    FileExtensionMsh,
    FileExtensionMsi,
    FileExtensionMsl,
    FileExtensionMsty,
    FileExtensionMts,
    FileExtensionMus,
    FileExtensionMusicxml,
    FileExtensionMvb,
    FileExtensionMwf,
    FileExtensionMxf,
    FileExtensionMxl,
    FileExtensionMxml,
    FileExtensionMxs,
    FileExtensionMxu,
    FileExtensionNGage,
    FileExtensionN3,
    FileExtensionNb,
    FileExtensionNbp,
    FileExtensionNc,
    FileExtensionNcx,
    FileExtensionNfo,
    FileExtensionNgdat,
    FileExtensionNitf,
    FileExtensionNlu,
    FileExtensionNml,
    FileExtensionNnd,
    FileExtensionNns,
    FileExtensionNnw,
    FileExtensionNpx,
    FileExtensionNsc,
    FileExtensionNsf,
    FileExtensionNtf,
    FileExtensionNzb,
    FileExtensionOa2,
    FileExtensionOa3,
    FileExtensionOas,
    FileExtensionObd,
    FileExtensionObj,
    FileExtensionOda,
    FileExtensionOdb,
    FileExtensionOdc,
    FileExtensionOdf,
    FileExtensionOdft,
    FileExtensionOdg,
    FileExtensionOdi,
    FileExtensionOdm,
    FileExtensionOdp,
    FileExtensionOds,
    FileExtensionOdt,
    FileExtensionOga,
    FileExtensionOgg,
    FileExtensionOgv,
    FileExtensionOgx,
    FileExtensionOmdoc,
    FileExtensionOnepkg,
    FileExtensionOnetmp,
    FileExtensionOnetoc,
    FileExtensionOnetoc2,
    FileExtensionOpf,
    FileExtensionOpml,
    FileExtensionOprc,
    FileExtensionOrg,
    FileExtensionOsf,
    FileExtensionOsfpvg,
    FileExtensionOtc,
    FileExtensionOtf,
    FileExtensionOtg,
    FileExtensionOth,
    FileExtensionOti,
    FileExtensionOtp,
    FileExtensionOts,
    FileExtensionOtt,
    FileExtensionOxps,
    FileExtensionOxt,
    FileExtensionP,
    FileExtensionP10,
    FileExtensionP12,
    FileExtensionP7b,
    FileExtensionP7c,
    FileExtensionP7m,
    FileExtensionP7r,
    FileExtensionP7s,
    FileExtensionP8,
    FileExtensionPas,
    FileExtensionPaw,
    FileExtensionPbd,
    FileExtensionPbm,
    FileExtensionPcap,
    FileExtensionPcf,
    FileExtensionPcl,
    FileExtensionPclxl,
    FileExtensionPct,
    FileExtensionPcurl,
    FileExtensionPcx,
    FileExtensionPdb,
    FileExtensionPdf,
    FileExtensionPfa,
    FileExtensionPfb,
    FileExtensionPfm,
    FileExtensionPfr,
    FileExtensionPfx,
    FileExtensionPgm,
    FileExtensionPgn,
    FileExtensionPgp,
    FileExtensionPic,
    FileExtensionPict,
    FileExtensionPkg,
    FileExtensionPki,
    FileExtensionPkipath,
    FileExtensionPlb,
    FileExtensionPlc,
    FileExtensionPlf,
    FileExtensionPls,
    FileExtensionPml,
    FileExtensionPng,
    FileExtensionPnm,
    FileExtensionPnt,
    FileExtensionPortpkg,
    FileExtensionPot,
    FileExtensionPotm,
    FileExtensionPotx,
    FileExtensionPpam,
    FileExtensionPpd,
    FileExtensionPpm,
    FileExtensionPps,
    FileExtensionPpsm,
    FileExtensionPpsx,
    FileExtensionPpt,
    FileExtensionPptm,
    FileExtensionPptx,
    FileExtensionPqa,
    FileExtensionPrc,
    FileExtensionPre,
    FileExtensionPrf,
    FileExtensionPs,
    FileExtensionPsb,
    FileExtensionPsd,
    FileExtensionPsf,
    FileExtensionPskcxml,
    FileExtensionPtid,
    FileExtensionPub,
    FileExtensionPvb,
    FileExtensionPwn,
    FileExtensionPya,
    FileExtensionPyv,
    FileExtensionQam,
    FileExtensionQbo,
    FileExtensionQfx,
    FileExtensionQps,
    FileExtensionQt,
    FileExtensionQti,
    FileExtensionQtif,
    FileExtensionQwd,
    FileExtensionQwt,
    FileExtensionQxb,
    FileExtensionQxd,
    FileExtensionQxl,
    FileExtensionQxt,
    FileExtensionRa,
    FileExtensionRam,
    FileExtensionRar,
    FileExtensionRas,
    FileExtensionRcprofile,
    FileExtensionRdf,
    FileExtensionRdz,
    FileExtensionRep,
    FileExtensionRes,
    FileExtensionRgb,
    FileExtensionRif,
    FileExtensionRip,
    FileExtensionRis,
    FileExtensionRl,
    FileExtensionRlc,
    FileExtensionRld,
    FileExtensionRm,
    FileExtensionRmi,
    FileExtensionRmp,
    FileExtensionRms,
    FileExtensionRmvb,
    FileExtensionRnc,
    FileExtensionRoa,
    FileExtensionRoff,
    FileExtensionRp9,
    FileExtensionRpss,
    FileExtensionRpst,
    FileExtensionRq,
    FileExtensionRs,
    FileExtensionRsd,
    FileExtensionRss,
    FileExtensionRtf,
    FileExtensionRtx,
    FileExtensionS,
    FileExtensionS3m,
    FileExtensionSaf,
    FileExtensionSbml,
    FileExtensionSc,
    FileExtensionScd,
    FileExtensionScm,
    FileExtensionScq,
    FileExtensionScs,
    FileExtensionScurl,
    FileExtensionSda,
    FileExtensionSdc,
    FileExtensionSdd,
    FileExtensionSdkd,
    FileExtensionSdkm,
    FileExtensionSdp,
    FileExtensionSdw,
    FileExtensionSee,
    FileExtensionSeed,
    FileExtensionSema,
    FileExtensionSemd,
    FileExtensionSemf,
    FileExtensionSer,
    FileExtensionSetpay,
    FileExtensionSetreg,
    FileExtensionSfdHdstx,
    FileExtensionSfs,
    FileExtensionSfv,
    FileExtensionSgi,
    FileExtensionSgl,
    FileExtensionSgm,
    FileExtensionSgml,
    FileExtensionSh,
    FileExtensionShar,
    FileExtensionShf,
    FileExtensionSid,
    FileExtensionSig,
    FileExtensionSil,
    FileExtensionSilo,
    FileExtensionSis,
    FileExtensionSisx,
    FileExtensionSit,
    FileExtensionSitx,
    FileExtensionSkd,
    FileExtensionSkm,
    FileExtensionSkp,
    FileExtensionSkt,
    FileExtensionSldm,
    FileExtensionSldx,
    FileExtensionSlt,
    FileExtensionSm,
    FileExtensionSmf,
    FileExtensionSmi,
    FileExtensionSmil,
    FileExtensionSmv,
    FileExtensionSmzip,
    FileExtensionSnd,
    FileExtensionSnf,
    FileExtensionSo,
    FileExtensionSpc,
    FileExtensionSpf,
    FileExtensionSpl,
    FileExtensionSpot,
    FileExtensionSpp,
    FileExtensionSpq,
    FileExtensionSpx,
    FileExtensionSql,
    FileExtensionSrc,
    FileExtensionSrt,
    FileExtensionSru,
    FileExtensionSrx,
    FileExtensionSsdl,
    FileExtensionSse,
    FileExtensionSsf,
    FileExtensionSsml,
    FileExtensionSt,
    FileExtensionStc,
    FileExtensionStd,
    FileExtensionStf,
    FileExtensionSti,
    FileExtensionStk,
    FileExtensionStl,
    FileExtensionStr,
    FileExtensionStw,
    FileExtensionSub,
    FileExtensionSus,
    FileExtensionSusp,
    FileExtensionSv4cpio,
    FileExtensionSv4crc,
    FileExtensionSvc,
    FileExtensionSvd,
    FileExtensionSvg,
    FileExtensionSvgz,
    FileExtensionSwa,
    FileExtensionSwf,
    FileExtensionSwi,
    FileExtensionSxc,
    FileExtensionSxd,
    FileExtensionSxg,
    FileExtensionSxi,
    FileExtensionSxm,
    FileExtensionSxw,
    FileExtensionT,
    FileExtensionT3,
    FileExtensionTaglet,
    FileExtensionTao,
    FileExtensionTar,
    FileExtensionTcap,
    FileExtensionTcl,
    FileExtensionTeacher,
    FileExtensionTei,
    FileExtensionTeicorpus,
    FileExtensionTex,
    FileExtensionTexi,
    FileExtensionTexinfo,
    FileExtensionText,
    FileExtensionTfi,
    FileExtensionTfm,
    FileExtensionTga,
    FileExtensionThmx,
    FileExtensionTif,
    FileExtensionTiff,
    FileExtensionTmo,
    FileExtensionTorrent,
    FileExtensionTpl,
    FileExtensionTpt,
    FileExtensionTr,
    FileExtensionTra,
    FileExtensionTrm,
    FileExtensionTsd,
    FileExtensionTsv,
    FileExtensionTtc,
    FileExtensionTtf,
    FileExtensionTtl,
    FileExtensionTwd,
    FileExtensionTwds,
    FileExtensionTxd,
    FileExtensionTxf,
    FileExtensionTxt,
    FileExtensionU32,
    FileExtensionUdeb,
    FileExtensionUfd,
    FileExtensionUfdl,
    FileExtensionUlw,
    FileExtensionUlx,
    FileExtensionUmj,
    FileExtensionUnityweb,
    FileExtensionUoml,
    FileExtensionUri,
    FileExtensionUris,
    FileExtensionUrls,
    FileExtensionUstar,
    FileExtensionUtz,
    FileExtensionUu,
    FileExtensionUva,
    FileExtensionUvd,
    FileExtensionUvf,
    FileExtensionUvg,
    FileExtensionUvh,
    FileExtensionUvi,
    FileExtensionUvm,
    FileExtensionUvp,
    FileExtensionUvs,
    FileExtensionUvt,
    FileExtensionUvu,
    FileExtensionUvv,
    FileExtensionUvva,
    FileExtensionUvvd,
    FileExtensionUvvf,
    FileExtensionUvvg,
    FileExtensionUvvh,
    FileExtensionUvvi,
    FileExtensionUvvm,
    FileExtensionUvvp,
    FileExtensionUvvs,
    FileExtensionUvvt,
    FileExtensionUvvu,
    FileExtensionUvvv,
    FileExtensionUvvx,
    FileExtensionUvvz,
    FileExtensionUvx,
    FileExtensionUvz,
    FileExtensionVcard,
    FileExtensionVcd,
    FileExtensionVcf,
    FileExtensionVcg,
    FileExtensionVcs,
    FileExtensionVcx,
    FileExtensionVis,
    FileExtensionViv,
    FileExtensionVob,
    FileExtensionVor,
    FileExtensionVox,
    FileExtensionVrml,
    FileExtensionVsd,
    FileExtensionVsf,
    FileExtensionVss,
    FileExtensionVst,
    FileExtensionVsw,
    FileExtensionVtu,
    FileExtensionVxml,
    FileExtensionW3d,
    FileExtensionWad,
    FileExtensionWav,
    FileExtensionWax,
    FileExtensionWbmp,
    FileExtensionWbs,
    FileExtensionWbxml,
    FileExtensionWcm,
    FileExtensionWdb,
    FileExtensionWdp,
    FileExtensionWeba,
    FileExtensionWebm,
    FileExtensionWebp,
    FileExtensionWg,
    FileExtensionWgt,
    FileExtensionWks,
    FileExtensionWm,
    FileExtensionWma,
    FileExtensionWmd,
    FileExtensionWmf,
    FileExtensionWml,
    FileExtensionWmlc,
    FileExtensionWmls,
    FileExtensionWmlsc,
    FileExtensionWmv,
    FileExtensionWmx,
    FileExtensionWmz,
    FileExtensionWoff,
    FileExtensionWoff2,
    FileExtensionWpd,
    FileExtensionWpl,
    FileExtensionWps,
    FileExtensionWqd,
    FileExtensionWri,
    FileExtensionWrl,
    FileExtensionWsdl,
    FileExtensionWspolicy,
    FileExtensionWtb,
    FileExtensionWvx,
    FileExtensionX32,
    FileExtensionX3d,
    FileExtensionX3db,
    FileExtensionX3dbz,
    FileExtensionX3dv,
    FileExtensionX3dvz,
    FileExtensionX3dz,
    FileExtensionXaml,
    FileExtensionXap,
    FileExtensionXar,
    FileExtensionXbap,
    FileExtensionXbd,
    FileExtensionXbm,
    FileExtensionXdf,
    FileExtensionXdm,
    FileExtensionXdp,
    FileExtensionXdssc,
    FileExtensionXdw,
    FileExtensionXenc,
    FileExtensionXer,
    FileExtensionXfdf,
    FileExtensionXfdl,
    FileExtensionXht,
    FileExtensionXhtml,
    FileExtensionXhvml,
    FileExtensionXif,
    FileExtensionXla,
    FileExtensionXlam,
    FileExtensionXlc,
    FileExtensionXlf,
    FileExtensionXlm,
    FileExtensionXls,
    FileExtensionXlsb,
    FileExtensionXlsm,
    FileExtensionXlsx,
    FileExtensionXlt,
    FileExtensionXltm,
    FileExtensionXltx,
    FileExtensionXlw,
    FileExtensionXm,
    FileExtensionXml,
    FileExtensionXo,
    FileExtensionXop,
    FileExtensionXpi,
    FileExtensionXpl,
    FileExtensionXpm,
    FileExtensionXpr,
    FileExtensionXps,
    FileExtensionXpw,
    FileExtensionXpx,
    FileExtensionXsl,
    FileExtensionXslt,
    FileExtensionXsm,
    FileExtensionXspf,
    FileExtensionXul,
    FileExtensionXvm,
    FileExtensionXvml,
    FileExtensionXwd,
    FileExtensionXyz,
    FileExtensionXz,
    FileExtensionYang,
    FileExtensionYin,
    FileExtensionZ,
    FileExtensionZ1,
    FileExtensionZ2,
    FileExtensionZ3,
    FileExtensionZ4,
    FileExtensionZ5,
    FileExtensionZ6,
    FileExtensionZ7,
    FileExtensionZ8,
    FileExtensionZaz,
    FileExtensionZip,
    FileExtensionZir,
    FileExtensionZirz,
    FileExtensionZmm,
    FileExtensionEmpty,
}
