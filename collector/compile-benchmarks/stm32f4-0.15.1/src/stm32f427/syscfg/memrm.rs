#[doc = "Register `MEMRM` reader"]
pub struct R(crate::R<MEMRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMRM` writer"]
pub struct W(crate::W<MEMRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMRM_SPEC>;
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
impl From<crate::W<MEMRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_MODE` reader - MEM_MODE"]
pub type MEM_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_MODE` writer - MEM_MODE"]
pub type MEM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEMRM_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - MEM_MODE"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MEM_MODE"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "memory remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memrm](index.html) module"]
pub struct MEMRM_SPEC;
impl crate::RegisterSpec for MEMRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memrm::R](R) reader structure"]
impl crate::Readable for MEMRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memrm::W](W) writer structure"]
impl crate::Writable for MEMRM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMRM to value 0"]
impl crate::Resettable for MEMRM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
