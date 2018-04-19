export default ({ Vue, router, options }) => {
  console.log(options)
  router.addRoutes([
    {
      path: '/archive',
      redirect: () => '/',
    }
  ])
}
