#[doc = "Register `KLR` writer"]
pub struct W(crate::W<KLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<KLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b2` writer - b224"]
pub type B2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KLR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - b224"]
    #[inline(always)]
    pub fn b2(&mut self) -> B2_W<0> {
        B2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [klr](index.html) module"]
pub struct KLR_SPEC;
impl crate::RegisterSpec for KLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [klr::W](W) writer structure"]
impl crate::Writable for KLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KLR to value 0"]
impl crate::Resettable for KLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
